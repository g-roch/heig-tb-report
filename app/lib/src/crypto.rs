use zkp::rand::CryptoRng;
use zkp::rand::RngCore;

use sha2;
use sha2::Sha512;

use zkp::Transcript;

use curve25519_dalek::constants as dalek_constants;
use curve25519_dalek::ristretto::CompressedRistretto;
use curve25519_dalek::ristretto::RistrettoPoint;
use curve25519_dalek::scalar::Scalar;

define_proof! {pr1, "app:proof1", (x), (ID, X), (G) : X = (x * G) }

#[allow(non_snake_case)]
pub mod pr2 {
    use zkp::curve25519_dalek::ristretto::CompressedRistretto;
    use zkp::curve25519_dalek::ristretto::RistrettoPoint;
    use zkp::curve25519_dalek::scalar::Scalar;

    use sha2::Sha512;

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
        pub A: &'a RistrettoPoint,
    }

    pub struct VerifyAssignments<'a> {
        pub ID: &'a CompressedRistretto,
        pub G: &'a CompressedRistretto,
        pub X: &'a Vec<CompressedRistretto>,
        pub Z: &'a Vec<CompressedRistretto>,
        pub Y: &'a Vec<CompressedRistretto>,
        pub A: &'a CompressedRistretto,
    }
    pub struct CompactProof {
        pub c: Vec<Scalar>,
        pub r: Vec<Scalar>,
        pub t: Vec<(CompressedRistretto, CompressedRistretto)>,
    }

    /// Hash data and return a Scalar associate to the hash
    /// It's used for check the proof is the proof of the public data associate
    fn hash(
        ID: &CompressedRistretto,
        G: &CompressedRistretto,
        X: &Vec<CompressedRistretto>,
        Y: &Vec<CompressedRistretto>,
        Z: &Vec<RistrettoPoint>,
        A: &RistrettoPoint,
        t: &Vec<(CompressedRistretto, CompressedRistretto)>,
    ) -> Scalar {
        let k = X.len();
        let mut hash_bytes: Vec<u8> = Default::default();
        hash_bytes.append(&mut Vec::from(*ID.as_bytes()));
        hash_bytes.append(&mut Vec::from(*G.as_bytes()));
        for l in 0..k {
            hash_bytes.append(&mut Vec::from(*X[l].as_bytes()));
            hash_bytes.append(&mut Vec::from(*Y[l].as_bytes()));
            hash_bytes.append(&mut Vec::from(*(Z[l] - A).compress().as_bytes()));
            hash_bytes.append(&mut Vec::from(*t[l].0.as_bytes()));
            hash_bytes.append(&mut Vec::from(*t[l].1.as_bytes()));
        }
        Scalar::hash_from_bytes::<Sha512>(&hash_bytes)
    }

    /// Create a partial proof valid ballot (k proof is required to validate ballot)
    pub fn prove_compact(assignments: ProveAssignments<'_>) -> CompactProof {
        let mut rng = thread_rng();

        let j = assignments.j;
        let k = assignments.X.len();
        let G = assignments.G;
        let A = assignments.A;
        let Y = assignments.Y;
        let X = assignments.X;
        let Z = assignments.Z;
        let x = assignments.x;
        let ID = assignments.ID.compress();

        let w = Scalar::random(&mut rng);
        // values c[j] and r[j] is override after, but is a random for simplify code:w
        let mut c: Vec<_> = (0..k).map(|_| Scalar::random(&mut rng)).collect();
        let mut r: Vec<_> = (0..k).map(|_| Scalar::random(&mut rng)).collect();

        let mut t: Vec<(_, _)> = (0..k)
            .map(|l| (r[l] * G + c[l] * X[l], r[l] * Y[l] + c[l] * (Z[l] - A)))
            .map(|(a, b)| (a.compress(), b.compress()))
            .collect();

        t[j] = ((w * G).compress(), (w * Y[j]).compress());

        let c_hash = hash(
            &ID,
            &G.compress(),
            &X.iter().map(RistrettoPoint::compress).collect(),
            &Y.iter().map(RistrettoPoint::compress).collect(),
            &Z,
            &A,
            &t,
        );

        c[j] = Scalar::zero();
        c[j] = c_hash - c.iter().sum::<Scalar>();
        r[j] = w - x * c[j];

        CompactProof { c, r, t }
    }

    /// Verify if a proof is valid
    ///
    /// If some point isn't a valid point, return Err(())
    pub fn verify_compact(proof: &CompactProof, assignments: VerifyAssignments) -> Result<(), ()> {
        let k = assignments.X.len();
        let A = assignments.A.decompress().ok_or(())?;

        let ID = assignments.ID;
        let G = assignments.G.decompress().ok_or(())?;
        let X: Vec<_> = assignments
            .X
            .iter()
            .map(CompressedRistretto::decompress)
            .collect::<Option<_>>()
            .ok_or(())?;
        let Y: Vec<_> = assignments
            .Y
            .iter()
            .map(CompressedRistretto::decompress)
            .collect::<Option<_>>()
            .ok_or(())?;
        let Z: Vec<_> = assignments
            .Z
            .iter()
            .map(CompressedRistretto::decompress)
            .collect::<Option<_>>()
            .ok_or(())?;
        let r = &proof.r;
        let c = &proof.c;

        let c_prime = hash(
            ID,
            &assignments.G,
            &assignments.X,
            &assignments.Y,
            &Z,
            &A,
            &proof.t,
        );

        let c_orig: Scalar = proof.c.iter().sum();
        if c_prime != c_orig {
            return Err(());
        }

        let t_prime: Vec<(CompressedRistretto, CompressedRistretto)> = (0..k)
            .map(|l| (r[l] * G + c[l] * X[l], r[l] * Y[l] + c[l] * (Z[l] - A)))
            .map(|(a, b)| (a.compress(), b.compress()))
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
    options: &'a Vec<u64>,

    /// Secret round 1
    x_i: Option<Vec<Scalar>>,

    /// Score for candidat
    v_i: Vec<u64>,
}

impl<'a, Rng> Crypto<'a, Rng>
where
    Rng: CryptoRng + RngCore + Clone,
{
    pub fn new(voterid: u64, rng: Rng, options: &'a Vec<u64>, v_i: Vec<u64>) -> Self {
        assert_eq!(v_i.len(), options.len());
        for v in &v_i {
            if !options.contains(&v) {
                eprintln!("WARN: {v} not in {options:?}");
            }
        }
        Self {
            voterid,
            rng,
            options,
            x_i: None,
            v_i,
        }
    }

    /// Make round 1 for the user
    #[allow(non_snake_case)]
    pub fn vote_round_1(&mut self) -> (Vec<CompressedRistretto>, Vec<pr1::CompactProof>) {
        let k = self.options.len();

        let x_i: Vec<Scalar> = (0..k)
            .map(|_| Scalar::random(&mut self.rng.clone()))
            .collect();
        self.x_i = Some(x_i.clone());

        let G = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
        let ID = RistrettoPoint::hash_from_bytes::<Sha512>(&self.voterid.to_le_bytes());

        x_i.iter()
            .enumerate()
            .map(|(j, x)| {
                let X = x * G;
                let mut transcript = Transcript::new(b"prove1");
                transcript.append_u64(b"j", j as u64);
                let (proof, _points) = pr1::prove_compact(
                    &mut transcript,
                    pr1::ProveAssignments {
                        ID: &ID,
                        X: &X,
                        x,
                        G,
                    },
                );
                (X.compress(), proof)
            })
            .unzip()
    }

    /// Verify NIZK of round 1
    #[allow(non_snake_case)]
    pub fn verify_round_1(
        round_1: &Vec<(Vec<CompressedRistretto>, Vec<pr1::CompactProof>)>,
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
                    .0
                    .iter()
                    .zip(round_1_i.1.iter())
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
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }

    /// Verify NIZK of round 2
    ///
    /// round_2 is deemed verified
    #[allow(non_snake_case)]
    fn verify_round_2(
        &self,
        round_1: &Vec<(Vec<CompressedRistretto>, Vec<pr1::CompactProof>)>,
        round_2: &Vec<(Vec<CompressedRistretto>, Vec<pr2::CompactProof>)>,
    ) -> Result<(), ()> {
        let G = &dalek_constants::RISTRETTO_BASEPOINT_POINT;

        let A: Vec<CompressedRistretto> = self
            .options
            .iter()
            .map(|opt| Scalar::from(*opt) * G)
            .map(|p| p.compress())
            .collect();

        // Verify all proof from round 2
        round_2
            .iter()
            .enumerate()
            .map(|(i_voterid, (Z, proofs))| {
                let ID =
                    RistrettoPoint::hash_from_bytes::<Sha512>(&i_voterid.to_le_bytes()).compress();
                let Y: Vec<_> = Self::calc_Yi(&round_1, i_voterid)
                    .ok_or(())?
                    .iter()
                    .map(RistrettoPoint::compress)
                    .collect();

                proofs
                    .iter()
                    .enumerate()
                    .map(|(j, proof)| {
                        pr2::verify_compact(
                            &proof,
                            pr2::VerifyAssignments {
                                ID: &ID,
                                X: &round_1[i_voterid].0,
                                Y: &Y,
                                G: &G.compress(),
                                A: &A[j],
                                Z,
                            },
                        )
                    })
                    .collect::<Result<Vec<_>, _>>()
            })
            .collect::<Result<Vec<_>, _>>()
            .map(|_| ())
    }

    #[allow(non_snake_case)]
    fn calc_Yij(
        round_1: &Vec<(Vec<CompressedRistretto>, Vec<pr1::CompactProof>)>,
        calc_i: usize,
        calc_j: usize,
    ) -> Option<RistrettoPoint> {
        let X: Vec<_> = round_1
            .iter()
            .map(|e| &e.0[calc_j])
            .map(CompressedRistretto::decompress)
            .collect::<Option<_>>()?;

        Some(
            X[..calc_i].iter().sum::<RistrettoPoint>()
                - X[(calc_i + 1)..].iter().sum::<RistrettoPoint>(),
        )
    }

    #[allow(non_snake_case)]
    fn calc_Yi(
        round_1: &Vec<(Vec<CompressedRistretto>, Vec<pr1::CompactProof>)>,
        calc_i: usize,
    ) -> Option<Vec<RistrettoPoint>> {
        let k = round_1.get(0)?.0.len();
        (0..k)
            .map(|j| Self::calc_Yij(&round_1, calc_i, j))
            .collect()
    }

    fn candidat_of_rank(&self, j: usize) -> Option<usize> {
        self.v_i.iter().position(|p| *p as usize == j)
    }

    #[allow(non_snake_case)]
    pub fn vote_round_2(
        &self,
        round_1: &Vec<(Vec<CompressedRistretto>, Vec<pr1::CompactProof>)>,
    ) -> Result<(Vec<CompressedRistretto>, Vec<pr2::CompactProof>), ()> {
        Self::verify_round_1(&round_1).map_err(|_| ())?;

        let ID = RistrettoPoint::hash_from_bytes::<Sha512>(&self.voterid.to_le_bytes());
        let G = &dalek_constants::RISTRETTO_BASEPOINT_POINT;
        let k = self.options.len();

        let Y_i: Vec<RistrettoPoint> = Self::calc_Yi(&round_1, self.voterid as usize).ok_or(())?;
        let X_i: Vec<_> = round_1[self.voterid as usize]
            .0
            .iter()
            .map(CompressedRistretto::decompress)
            .collect::<Option<_>>()
            .ok_or(())?;
        let x_i = self.x_i.clone().ok_or(())?;
        let Z_i: Vec<_> = (0..k)
            .map(|j| x_i[j] * Y_i[j] + Scalar::from(self.v_i[j]) * G)
            .collect();
        let A: Vec<_> = (0..k).map(|j| Scalar::from(self.options[j]) * G).collect();

        let proofs = A
            .iter()
            .enumerate()
            .map(|(m, A_m)| {
                let j = self.candidat_of_rank(m)?;
                let proof = pr2::prove_compact(pr2::ProveAssignments {
                    x: &x_i[j],
                    j,
                    ID: &ID,
                    G,
                    X: &X_i,
                    Z: &Z_i,
                    Y: &Y_i,
                    A: &A_m,
                });
                Some(proof)
            })
            .collect::<Option<_>>()
            .ok_or(())?;
        let Z_i_compressed = Z_i.iter().map(RistrettoPoint::compress).collect();
        Ok((Z_i_compressed, proofs))
    }

    #[allow(non_snake_case)]
    pub fn tallying(
        &self,
        round_1: &Vec<(Vec<CompressedRistretto>, Vec<pr1::CompactProof>)>,
        round_2: &Vec<(Vec<CompressedRistretto>, Vec<pr2::CompactProof>)>,
    ) -> Result<Vec<u64>, ()> {
        Self::verify_round_1(&round_1).map_err(|_| ())?;
        self.verify_round_2(&round_1, &round_2)?;

        let k = self.options.len();
        let G = dalek_constants::RISTRETTO_BASEPOINT_POINT;
        let Z: Vec<Vec<RistrettoPoint>> = round_2
            .iter()
            .map(|round_2_i| {
                round_2_i
                    .0
                    .iter()
                    .map(CompressedRistretto::decompress)
                    .collect::<Option<_>>()
            })
            .collect::<Option<_>>()
            .ok_or(())?;

        let R: Vec<RistrettoPoint> = (0..k)
            .map(|j| Z.iter().map(move |Z_i| Z_i[j]).sum())
            .collect();

        // TODO implement faster algorithm
        let result: Vec<_> = R
            .iter()
            .map(|&R_j| {
                let mut r = 0u64;
                let mut tmp = Scalar::from(r) * G;
                loop {
                    if tmp == R_j {
                        break r;
                    }
                    tmp += G;
                    r += 1;
                }
            })
            .collect();

        Ok(result)
    }
}
