use zkp::rand::seq::SliceRandom;
use zkp::rand::CryptoRng;
use zkp::rand::RngCore;

use sha2;
use sha2::Digest;
use sha2::Sha512;

use zkp::Transcript;

use curve25519_dalek::constants as dalek_constants;
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

use crate::VoteOption;

define_proof! {pr1, "app:proof1", (x), (ID, X), (G) : X = (x * G) }
//define_proof! {pr2, "app:proof2", (x, v), (ID, X, V), (G, Y) : X = (x * G), V  = (x * Y + v * G) }

#[allow(non_snake_case)]
mod pr2 {
    use zkp::curve25519_dalek::ristretto::CompressedRistretto;
    use zkp::curve25519_dalek::ristretto::RistrettoPoint;
    use zkp::curve25519_dalek::scalar::Scalar;

    use sha2::Sha512;
    use zkp::merlin::Transcript;

    use zkp::rand::thread_rng;

    #[derive(Copy, Clone)]
    pub struct ProveAssignments<'a> {
        pub x: &'a Scalar,
        pub j: usize,
        pub ID: &'a RistrettoPoint,
        pub G: &'a RistrettoPoint,
        pub X: &'a Vec<RistrettoPoint>,
        pub Z: &'a Vec<RistrettoPoint>,
        pub Y: &'a Vec<RistrettoPoint>,
        pub V: &'a RistrettoPoint,
    }

    pub struct CompressedPoints {}
    pub struct VerifyAssignments<'a> {
        pub ID: &'a CompressedRistretto,
        pub G: &'a CompressedRistretto,
        pub X: &'a Vec<CompressedRistretto>,
        pub Z: &'a Vec<CompressedRistretto>,
        pub Y: &'a Vec<CompressedRistretto>,
        pub V: &'a CompressedRistretto,
    } // CompressedPoints but by ref
    pub struct CompactProof {
        pub c: Vec<Scalar>,
        pub r: Vec<Scalar>,
        pub t: Vec<(CompressedRistretto, CompressedRistretto)>,
    }

    fn hash(
        ID: &CompressedRistretto,
        G: &CompressedRistretto,
        X: &Vec<CompressedRistretto>,
        Y: &Vec<CompressedRistretto>,
        Z: &Vec<RistrettoPoint>,
        V: &RistrettoPoint,
        t: &Vec<(CompressedRistretto, CompressedRistretto)>,
    ) -> Scalar {
        let k = X.len();
        let mut hash_bytes: Vec<u8> = Default::default();
        hash_bytes.append(&mut Vec::from(*ID.as_bytes()));
        hash_bytes.append(&mut Vec::from(*G.as_bytes()));
        for l in 0..k {
            hash_bytes.append(&mut Vec::from(*X[l].as_bytes()));
            hash_bytes.append(&mut Vec::from(*Y[l].as_bytes()));
            hash_bytes.append(&mut Vec::from(*(Z[l] - V).compress().as_bytes()));
            hash_bytes.append(&mut Vec::from(*t[l].0.as_bytes()));
            hash_bytes.append(&mut Vec::from(*t[l].1.as_bytes()));
        }
        Scalar::hash_from_bytes::<Sha512>(&hash_bytes)
    }

    pub fn prove_compact(
        transcript: &mut Transcript,
        assignments: ProveAssignments<'_>,
    ) -> (CompactProof, CompressedPoints) {
        // Construct a TranscriptRng
        // This paragraphe is inspired from crate 'zkp' file 'src/toolbox/prover.rs' fn 'prove_impl'
        // under license CC0-1.0 (copied 2022-06-05)
        let mut rng_builder = transcript.build_rng();
        rng_builder = rng_builder.rekey_with_witness_bytes(b"", assignments.x.as_bytes());
        rng_builder = rng_builder
            .rekey_with_witness_bytes(b"", Scalar::from(assignments.j as u64).as_bytes());
        let mut transcript_rng = rng_builder.finalize(&mut thread_rng());

        let k = assignments.X.len();

        let w = Scalar::random(&mut transcript_rng);

        // values c[j] and r[j] is override after, but is a random for simplify code:w
        let mut c: Vec<_> = (0..k)
            .map(|_| Scalar::random(&mut transcript_rng))
            .collect();
        let mut r: Vec<_> = (0..k)
            .map(|_| Scalar::random(&mut transcript_rng))
            .collect();

        let mut t: Vec<(_, _)> = (0..k)
            .map(|l| {
                (
                    (r[l] * assignments.G + c[l] * assignments.X[l]).compress(),
                    (r[l] * assignments.Y[l] + c[l] * (assignments.Z[l] - assignments.V))
                        .compress(),
                )
            })
            .collect();
        t[assignments.j] = (
            (w * assignments.G).compress(),
            (w * assignments.Y[assignments.j]).compress(),
        );

        let c_hash = hash(
            &assignments.ID.compress(),
            &assignments.G.compress(),
            &assignments.X.iter().map(|e| e.compress()).collect(),
            &assignments.Y.iter().map(|e| e.compress()).collect(),
            &assignments.Z,
            &assignments.V,
            &t,
        );

        c[assignments.j] = Scalar::zero();
        c[assignments.j] = c_hash - c.iter().sum::<Scalar>();
        // TODO Pas sur de la multiplication ↓
        r[assignments.j] = w - assignments.x * c[assignments.j];

        (CompactProof { c, r, t }, CompressedPoints {})
    }

    pub fn verify_compact(
        proof: &CompactProof,
        transcript: &mut Transcript,
        assignments: VerifyAssignments,
    ) -> Result<(), ()> {
        let k = assignments.X.len();
        let c_prime = hash(
            &assignments.ID,
            &assignments.G,
            &assignments.X,
            &assignments.Y,
            &assignments
                .Z
                .iter()
                .map(|e| e.decompress().unwrap())
                .collect(),
            &assignments.V.decompress().unwrap(),
            &proof.t,
        );
        let c_orig: Scalar = proof.c.iter().sum();
        if c_prime != c_orig {
            return Err(());
        }

        let G = assignments.G.decompress().unwrap();
        let V = assignments.V.decompress().unwrap();
        let X: Vec<RistrettoPoint> = assignments
            .X
            .iter()
            .map(|p| p.decompress().unwrap())
            .collect();
        let Y: Vec<RistrettoPoint> = assignments
            .Y
            .iter()
            .map(|p| p.decompress().unwrap())
            .collect();
        let Z: Vec<RistrettoPoint> = assignments
            .Z
            .iter()
            .map(|p| p.decompress().unwrap())
            .collect();

        let t_prime: Vec<(CompressedRistretto, CompressedRistretto)> = (0..k)
            .map(|l| {
                (
                    (proof.r[l] * G + proof.c[l] * X[l]).compress(),
                    (proof.r[l] * Y[l] + proof.c[l] * (Z[l] - V)).compress(),
                )
            })
            .collect();

        if t_prime == proof.t {
            Ok(())
        } else {
            Err(())
        }
    }
}

pub struct Crypto<'a, Rng>
where
    Rng: CryptoRng + RngCore + Clone,
{
    /// user identifier
    voterid: u64,

    /// Random generator
    rng: Rng,

    /// Options
    options: &'a Vec<String>,

    /// Number of candidates
    k_candidate_count: usize,

    /// Secret round 1
    x_i: Option<Vec<Scalar>>,

    /// Score for candidat
    v_i: Vec<u64>,
}

impl<'a, Rng> Crypto<'a, Rng>
where
    Rng: CryptoRng + RngCore + Clone,
{
    pub fn new(voterid: u64, rng: Rng, options: &'a Vec<String>, v_i: Vec<u64>) -> Self {
        let k_candidate_count = options.len();
        assert_eq!(v_i.len(), options.len());
        //let mut v_i: Vec<_> = (0..k_candidate_count as u64).collect();
        //let mut rng = rng_;
        //v_i.shuffle(&mut rng);
        Self {
            voterid,
            rng,
            options,
            k_candidate_count,
            x_i: None,
            v_i,
        }
    }

    #[allow(non_snake_case)]
    pub fn vote_round_1(&mut self) -> Vec<(CompressedRistretto, pr1::CompactProof)> {
        let x_i: Vec<Scalar> = (0..self.k_candidate_count)
            .map(|_| Scalar::random(&mut self.rng.clone()))
            .collect();
        self.x_i = Some(x_i.clone());

        let G: &RistrettoPoint = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
        let ID = RistrettoPoint::hash_from_bytes::<Sha512>(&self.voterid.to_le_bytes());

        x_i.iter()
            .enumerate()
            .map(|(j, x)| {
                let X = x * G;
                let mut transcript = Transcript::new(b"prove1");
                transcript.append_u64(b"j", j as u64);
                let (proof, points) = pr1::prove_compact(
                    &mut transcript,
                    pr1::ProveAssignments {
                        ID: &ID,
                        X: &X,
                        x,
                        G,
                    },
                );
                assert_eq!(&points.G, &dalek_constants::RISTRETTO_BASEPOINT_COMPRESSED);
                assert_eq!(&points.ID, &ID.compress());
                (points.X, proof)
            })
            .collect::<Vec<_>>()
    }

    #[allow(non_snake_case)]
    fn verify_round_1(
        round_1: &Vec<Vec<(CompressedRistretto, pr1::CompactProof)>>,
    ) -> Result<(), pr1::ProofError> {
        let G = &dalek_constants::RISTRETTO_BASEPOINT_COMPRESSED;

        // Verify all proof from round 1
        round_1
            .iter()
            .enumerate()
            .map(|(voterid, round_1_i)| {
                let ID =
                    RistrettoPoint::hash_from_bytes::<Sha512>(&voterid.to_le_bytes()).compress();
                round_1_i
                    .iter()
                    .enumerate()
                    .map(|(j, (X, proof))| {
                        let mut transcript = Transcript::new(b"prove1");
                        transcript.append_u64(b"j", j as u64);
                        pr1::verify_compact(
                            &proof,
                            &mut transcript,
                            pr1::VerifyAssignments { ID: &ID, X, G },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    #[allow(non_snake_case)]
    fn verify_round_2(
        X: &Vec<CompressedRistretto>,
        round_2: &Vec<Vec<(CompressedRistretto, pr2::CompactProof)>>,
    ) -> Result<(), ()> {
        let G = &dalek_constants::RISTRETTO_BASEPOINT_COMPRESSED;

        // Verify all proof from round 2
        round_2
            .iter()
            .enumerate()
            .map(|(voterid, round_2_i)| {
                let ID =
                    RistrettoPoint::hash_from_bytes::<Sha512>(&voterid.to_le_bytes()).compress();
                round_2_i
                    .iter()
                    .enumerate()
                    .map(|(j, (Z, proof))| {
                        let mut transcript = Transcript::new(b"prove1");
                        transcript.append_u64(b"j", j as u64);
                        pr2::verify_compact(
                            &proof,
                            &mut transcript,
                            pr2::VerifyAssignments { ID: &ID, X: X, G },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(())
    }

    fn calc_Y(
        round_1: &Vec<Vec<(CompressedRistretto, pr1::CompactProof)>>,
        i: usize,
        j: usize,
    ) -> RistrettoPoint {
        let mut Y_ij: RistrettoPoint = Default::default();
        Y_ij += round_1[..(i)]
            .iter()
            .map(|e| e[j].0.decompress().unwrap())
            .sum::<RistrettoPoint>();
        Y_ij -= round_1[(i + 1)..]
            .iter()
            .map(|e| e[j].0.decompress().unwrap())
            .sum::<RistrettoPoint>();
        Y_ij
    }

    #[allow(non_snake_case)]
    pub fn vote_round_2(
        &self,
        round_1: &Vec<Vec<(CompressedRistretto, pr1::CompactProof)>>,
    ) -> Result<Vec<(CompressedRistretto, pr2::CompactProof)>, pr1::ProofError> {
        Self::verify_round_1(&round_1)?;

        let ID = RistrettoPoint::hash_from_bytes::<Sha512>(&self.voterid.to_le_bytes());
        let G = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
        let Y_i: Vec<RistrettoPoint> = (0..self.k_candidate_count)
            .map(|j| Self::calc_Y(&round_1, self.voterid as usize, j))
            .collect();

        let x_i = self.x_i.clone().unwrap();
        let V_i: Vec<_> = self.v_i.iter().map(|v| Scalar::from(*v) * G).collect();
        let Z_i: Vec<_> = (0..self.k_candidate_count)
            .map(|j| x_i[j] * Y_i[j] + V_i[j])
            //.map(|Z| Z.compress())
            .collect();

        Ok(Z_i
            .iter()
            .enumerate()
            .map(|(j, Z)| {
                let mut transcript = Transcript::new(b"prove2");
                transcript.append_u64(b"j", j as u64);
                let (proof, points) = pr2::prove_compact(
                    &mut transcript,
                    pr2::ProveAssignments {
                        x: &x_i[j],
                        j,
                        ID: &ID,
                        G,
                        X: &round_1[self.voterid as usize]
                            .iter()
                            .map(|i| i.0.decompress().unwrap())
                            .collect(),
                        Z: &Z_i,
                        Y: &Y_i,
                        V: &V_i[j],
                    },
                );
                (Z.compress(), proof)
            })
            .collect::<Vec<_>>())
    }

    #[allow(non_snake_case)]
    pub fn tallying(
        &self,
        round_1: &Vec<Vec<(CompressedRistretto, pr1::CompactProof)>>,
        round_2: &Vec<Vec<(CompressedRistretto, pr2::CompactProof)>>,
    ) -> Result<Vec<u64>, ()> {
        Self::verify_round_1(&round_1).map_err(|_| ())?;
        Self::verify_round_2(&round_2).map_err(|_| ())?;

        let G = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
        let Z: Vec<Vec<RistrettoPoint>> = round_2
            .iter()
            .map(|r_i| {
                r_i.iter()
                    .map(|r_ij| r_ij.0.decompress().unwrap())
                    .collect()
            })
            .collect();

        let R: Vec<RistrettoPoint> = (0..self.k_candidate_count)
            .map(|j| Z.iter().map(move |Z_i| Z_i[j]).sum())
            .collect();

        // TODO implement faster algorithm
        let result: Vec<_> = (0..self.k_candidate_count)
            .map(|j| {
                let mut tmp = 0u64;
                loop {
                    if Scalar::from(tmp) * G == R[j] {
                        break tmp;
                    }
                    tmp += 1;
                }
            })
            .collect();

        Ok(result)
    }
}

//#[test]
//fn create_and_verify_compact() {
//    // Prover's scope
//    let (proof, points) = {
//        let ID = RistrettoPoint::hash_from_bytes::<Sha512>(b"ID de mon utilisateur");
//        let x = Scalar::from(89327492234u64).invert();
//        let X = &x * &dalek_constants::RISTRETTO_BASEPOINT_TABLE;
//
//        let mut transcript = Transcript::new(b"DLEQTest");
//        pr1::prove_compact(
//            &mut transcript,
//            pr1::ProveAssignments {
//                ID: &ID,
//                X: &x,
//                A: &X,
//                G: &dalek_constants::RISTRETTO_BASEPOINT_POINT,
//            },
//        )
//    };
//
//    // Serialize and parse bincode representation
//    let proof_bytes = bincode::serialize(&proof).unwrap();
//    let parsed_proof: pr1::CompactProof = bincode::deserialize(&proof_bytes).unwrap();
//
//    // Verifier logic
//    let mut transcript = Transcript::new(b"DLEQText");
//    assert!(pr1::verify_compact(
//        &parsed_proof,
//        &mut transcript,
//        pr1::VerifyAssignments {
//            X: &points.X,
//            ID: &points.ID,
//            G: &dalek_constants::RISTRETTO_BASEPOINT_COMPRESSED,
//            //H: &RistrettoPoint::hash_from_bytes::<Sha512>(b"A VRF input, for instance").compress(),
//        },
//    )
//    .is_ok());
//}

//// type Hasher = sha2::Sha512;
////
//// type TheCurve = NistP256;
//// type ThePoint = <TheCurve as ProjectiveArithmetic>::ProjectivePoint;
////
////
//// impl<Rng> Crypto<Rng>
//// where
////     Rng: CryptoRng + RngCore,
//// {
////     pub fn new(rng: Rng, k_candidate_number: usize) -> Self {
////         Self {
////             rng,
////             k_candidate_number,
////         }
////     }
////     fn H(&self, voterid: &str, g: i8, X: i8, t1: i8) -> Vec<u8> {
////         let mut hasher = Hasher::new();
////         hasher.update(&voterid);
////         //hasher.update(&[g, X, t1]);
////         hasher.finalize().to_vec()
////     }
////
////     pub fn vote(&mut self) {
////         let G_generator = ThePoint::GENERATOR;
////         let q_order = TheCurve::ORDER;
////         let x_secrets: Vec<_> = (0..self.k_candidate_number)
////             .map(|_| ThePoint::random(&mut self.rng))
////             .collect();
////         let mut v_vote_choose: Vec<usize> = (0usize..self.k_candidate_number).collect();
////         v_vote_choose.shuffle(&mut self.rng);
////
////         // first round
////
////         let X_publics: Vec<_> = x_secrets.iter().map(|x| G_generator * x).collect();
////     }
////
////     //// /// TODO Attention NonZeroScalar à la place de Scalar, est-ce correct de manière crypto
////     //// fn generate_xi(&mut self) -> Vec<ProjectivePoint> {
////     ////     let mut xi = vec![];
////     ////     for _ in 0..self.k {
////     ////         xi.push(ProjectivePoint::random(&mut self.rng));
////     ////     }
////     ////     xi
////     //// }
////
////     //// // prove of X = g^x
////     //// fn prover_1(&mut self, voterid: &str, X: ProjectivePoint, x: ProjectivePoint) -> (i8, i8) {
////     ////     let G = AffinePoint::GENERATOR;
////     ////     let n: usize = 0;
////
////     ////     let k = Scalar::random(&mut self.rng);
////     ////     let kG = k * G;
////
////     ////     //let w = Scalar::random(&mut self.rng);
////     ////     ////let w = ProjectivePoint::random(&mut self.rng);
////     ////     //let t1 = g * w;
////     ////     //let c: Scalar = self.H(voterid, g, X, t1);
////     ////     //let r = -(x * c) + w;
////     ////     ////let r = w - c * x;
////     ////     //return (r, t1);
////     //// }
////
////     //// fn verifier_1(&self, voterid: &str, xG: i8, prove: (i8, i8)) -> Result<(), ()> {
////     ////     let (r, t1) = prove;
////     ////     let G = ProjectivePoint::GENERATOR;
////     ////     let c = self.H(voterid, g, X, t1);
////     ////     let t1_prime = g ^ r; //* X ^ c;
////     ////     t1 == t1_prime
////     //// }
////
////     //fn prove_2(&self, voterid: &str, g: i8, k: i8,
//// }
////
//// //fn prove_2(
////
//// #[cfg(test)]
//// mod tests {
////     #[test]
////     fn it_works() {
////         let result = 2 + 2;
////         assert_eq!(result, 4);
////     }
//// }
