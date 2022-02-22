%! Class = heig-bachelor
%! Author = Gaby Roch
%! Date = 15.10.21

\NeedsTeXFormat{LaTeX2e}
\ProvidesClass{heig-bachelor}[2020/10/15 Unofficial template for bachelor on HEIG-VD]

% if I need compteur: chap. 4.12, p.61

\DeclareOption*{\PassOptionsToClass{\CurrentOption}{book}}
\ProcessOptions\relax

\LoadClass{book}


\RequirePackage{xifthen}
\RequirePackage{tabularx}
\RequirePackage{babel}
\RequirePackage{graphicx}
\RequirePackage{multirow}
\RequirePackage{makecell}
\RequirePackage{fancyhdr}
%\RequirePackage[headings]{fullpage}
\RequirePackage[a4paper,margin=1.25cm,top=2.5cm,headsep=1cm,bottom=2.25cm]{geometry}
\RequirePackage{emptypage}
\RequirePackage{shorttoc}
\RequirePackage{color}
\RequirePackage[colorlinks,unicode]{hyperref}
\RequirePackage[most]{tcolorbox}
\RequirePackage{fancyvrb}
\RequirePackage[tikz]{bclogo}
\RequirePackage{listingsutf8}
\RequirePackage{heig}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% en-tête
%
\pagestyle{fancy}
%\newgeometry{}

\fancyhead[RE,LO]{\includegraphics[height=1cm]{bachelor-school-slim}}
\fancyhead[RO]{\@ifconfidential\\{\scriptsize\@version} \textbf{\@shorttitle}}
\fancyhead[LE]{\@ifconfidential\\\textbf{\@shorttitle} {\scriptsize\@version}}

\fancypagestyle{plain}{
    \fancyhead[RE,LO]{\includegraphics[height=1cm]{bachelor-school-slim}}
    \fancyhead[RO]{\@ifconfidential\\{\scriptsize\@version} \textbf{\@shorttitle}}
    \fancyhead[LE]{\@ifconfidential\\\textbf{\@shorttitle} {\scriptsize\@version}}
}


%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% String d'affichage
%
\newcommand{\bachelorname}{Bachelor thesis}
\newcommand{\confidentialname}{Confidential}
\newcommand{\nonconfidentialname}{Non-confidential}
\newcommand{\thedatename}{}
\newcommand{\departmentname}{Department}
\newcommand{\facultyname}{Faculty}
\newcommand{\orientationname}{Orientation}
\newcommand{\proposedbyname}{Work proposed by}
\newcommand{\studentname}{Studant}
\newcommand{\responsibleteachername}{Responsible teacher}
\newcommand{\academicyearname}{Academic year}
\newcommand{\publishablesummaryname}{Publishable summary}
\newcommand{\summaryname}{Summary}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% traduction française
%
\addto\captionsfrench{
    \renewcommand{\bachelorname}{Travail de Bachelor}
    \renewcommand{\confidentialname}{Confidentiel}
    \renewcommand{\nonconfidentialname}{Non confidentiel}
    \renewcommand{\thedatename}{le\ }
    \renewcommand{\departmentname}{Département}
    \renewcommand{\facultyname}{Filière}
    \renewcommand{\orientationname}{Orientation}
    \renewcommand{\proposedbyname}{Travail proposé par}
    \renewcommand{\studentname}{Étudiant\if@authorgenderfem e\fi}
    \renewcommand{\responsibleteachername}{Enseignant\if@teachergenderfem e\fi\ responsable}
    \renewcommand{\academicyearname}{Année académique}
    \renewcommand{\publishablesummaryname}{Résumé publiable}
    \renewcommand{\summaryname}{Sommaire}
}

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% définition commande de rendu
%
% \summary[profondeur=1] -- Affiche un sommaire de profondeur `profondeur`
\newcommand{\summary}[1][1]{\shorttableofcontents{\summaryname}{#1}}

% \maketitle -- Affiche la page de titre
\renewcommand\maketitle{
    \newgeometry{margin=2cm}
    \begin{titlepage}%
        \let\footnotesize\small
        \let\footnoterule\relax
        \let \footnote \thanks
        \noindent
        \begin{tabularx}{\linewidth}{Xr}
          \multirow{3}{*}{\includegraphics[height=4em]{bachelor-school}} & \departmentname\ \@shortdepartment \\[0.5em]
            & \facultyname\ \@faculty            \\[0.5em]
            & \orientationname\ \@orientation
        \end{tabularx}
        \vskip 60\p@
        \begin{center}%
        {\LARGE \bachelorname \par}%
            \vskip 2em%
                {\Large \@title \\ \@confidential \par}%
            \vskip 1em%
                {\Large \@version \par}%
            \vskip 2em%
            \if@illustration
            \includegraphics[height=\@illustrationheight]{\@illustration}
            \else
            \vskip 10em
            \fi
            \vskip 2em
            \begin{tabularx}{\linewidth}{lX}
              \textbf{\studentname~:}            & \textbf{\@author}                \\[0.5em]
                \if@proposedby
                \textbf{\proposedbyname~:}         & \@proposedbyname                 \\[0.5em]
                ~                                  & \@proposedbyinst                 \\[0.5em]
                ~                                  & \makecell[l]{\@proposedbyadress} \\[0.5em]
                \fi % TODO: change vspace for allow newlign in value
                \textbf{\responsibleteachername~:} & \@teacher                        \\[0.5em]
                \textbf{\academicyearname~:}       & \@schoolyear                     \\[0.5em]
            \end{tabularx}
            \if@illustration
            \vskip 2em
            \else
            \vskip 10em
            \fi
            \global\let\@tmpvskip\@empty
            \vfil
            \begin{flushright}
                \@location, \thedatename\@date
            \end{flushright}
        \end{center}\par
        \@thanks
        \vfil\null
    \end{titlepage}%
    \setcounter{footnote}{0}%
    \restoregeometry
}


\newenvironment{abstract}{
    \cleardoublepage
    \begin{flushright}\small
        \departmentname\ \@shortdepartment   \\[0.5em]
        \facultyname\ \@faculty         \\[0.5em]
        \orientationname\ \@orientation \\[0.5em]
        \studentname\ \@author \\[0.5em]
        \responsibleteachername\ \@teacher\\[0.5em]
    \end{flushright}
\begin{center}
\vskip 3em
{\large \bachelorname\ }\@schoolyear\\[0.5em]
\@title\\[1em]
\hrulefill\\
\end{center}
\vspace*{\fill}
\noindent\textbf{\large \publishablesummaryname}\vskip 1em
}{
\par\vspace*{\fill}
\noindent
\begin{tabularx}{\linewidth}{XXX}
Étudiant :               & Date et lieu : & Signature : \\[0.5em]
\@author                 & \hrulefill     & \hrulefill  \\[1em]  % TODO baiser la ligne
Enseignant responsable : & Date et lieu : & Signature : \\[0.5em]
\@teacher                & \hrulefill     & \hrulefill  \\[1em]
\if@proposedby
\@proposedbyinst :       & Date et lieu : & Signature : \\[0.5em]
\@proposedbyname         & \hrulefill     & \hrulefill  \\[1em]
\fi
\end{tabularx}
}



\newenvironment{signedpage}[4]{
\def\@signedpageAuthor{#2}
\def\@signedpageLocation{#3}
\def\@signedpageDate{#4}
\cleardoublepage
\vspace*{\fill}
\begin{center}
\noindent{\huge \bfseries #1}\par\vskip 2em
\end{center}
\begin{quote}
}{
\end{quote}
\par
\vskip 5em
\noindent\begin{tabularx}{\linewidth}{XX}
~ & \makecell[l]{\@signedpageAuthor}
\end{tabularx}
\par
\vskip 10em
\begin{quote}
\@signedpageLocation, \thedatename \@signedpageDate \\
\end{quote}
\vspace*{\fill}
}


\renewcommand\bcStyleTitre[1]{\large\textbf{~#1}}
\renewcommand\bcStyleSousTitre[1]{\normalsize\textit{#1}}
\setlength{\logowidth}{30pt}

\newenvironment{pictonote2}[3]{%
\begin{center}%
\begin{minipage}{0.85\linewidth}%
\begin{bclogo}[arrondi=0.1,%
  marge=11,%
  sousTitre={#2},%
  logo={\includegraphics[width=\logowidth]{#3}},%
  ombre=true,%
  blur,%
  barre=snake,%
  tailleOndu=4.5,%
  ]{#1}%
}{%
\end{bclogo}%
\end{minipage}%
\end{center}%
}




\newlength{\largeurnota}
\newlength{\paddingnota}
\newlength{\indentationnota}
\newlength{\largeurlignenota}

\setlength{\largeurnota}{3.8em}
\setlength{\paddingnota}{0.3em}

% From https://framabook.org/tout-sur-latex/ on Art Libre (http://artlibre.org/)
\newenvironment{pictonote}[1]{%
\begin{list}{}{%
\setlength{\labelsep}{0pt}%
\setlength{\rightmargin}{15pt}}
\item%
\setlength{\indentationnota}{%
\@totalleftmargin+\largeurnota+\paddingnota}%
\setlength{\largeurlignenota}{%
\linewidth-\largeurnota-\paddingnota}%
\parshape=4%
\indentationnota\largeurlignenota%
\indentationnota\largeurlignenota%
\indentationnota\largeurlignenota%
\@totalleftmargin\linewidth%
\raisebox{-\largeurnota+2.2ex}[0pt][0pt]{%
\makebox[0pt][r]{%
\includegraphics[width=\largeurnota]{#1}%
\hspace{\paddingnota}}}%
\ignorespaces}{%
\end{list}}

%\newenvironment{nota}{\begin{pictonote}{notice}}{\end{pictonote}}
%\newenvironment{question}{\begin{pictonote}{question}}{\end{pictonote}}
%\newenvironment{important}{\begin{pictonote}{important}}{\end{pictonote}}
%\newenvironment{warning}{\begin{pictonote}{warning}}{\end{pictonote}}

\newenvironment{nota}[2][]{\begin{pictonote2}{#2}{#1}{notice}}{\end{pictonote2}}
\newenvironment{question}[2][]{\begin{pictonote2}{#2}{#1}{question}}{\end{pictonote2}}
\newenvironment{important}[2][]{\begin{pictonote2}{#2}{#1}{important}}{\end{pictonote2}}
\newenvironment{warning}[2][]{\begin{pictonote2}{#2}{#1}{warning}}{\end{pictonote2}}
%\newenvironment{question}{\begin{pictonote}{question}}{\end{pictonote}}
%\newenvironment{important}{\begin{pictonote}{important}}{\end{pictonote}}
%\newenvironment{warning}{\begin{pictonote}{warning}}{\end{pictonote}}

\lstset{literate=
  {á}{{\'a}}1 {é}{{\'e}}1 {í}{{\'i}}1 {ó}{{\'o}}1 {ú}{{\'u}}1
  {Á}{{\'A}}1 {É}{{\'E}}1 {Í}{{\'I}}1 {Ó}{{\'O}}1 {Ú}{{\'U}}1
  {à}{{\`a}}1 {è}{{\`e}}1 {ì}{{\`i}}1 {ò}{{\`o}}1 {ù}{{\`u}}1
  {À}{{\`A}}1 {È}{{\'E}}1 {Ì}{{\`I}}1 {Ò}{{\`O}}1 {Ù}{{\`U}}1
  {ä}{{\"a}}1 {ë}{{\"e}}1 {ï}{{\"i}}1 {ö}{{\"o}}1 {ü}{{\"u}}1
  {Ä}{{\"A}}1 {Ë}{{\"E}}1 {Ï}{{\"I}}1 {Ö}{{\"O}}1 {Ü}{{\"U}}1
  {â}{{\^a}}1 {ê}{{\^e}}1 {î}{{\^i}}1 {ô}{{\^o}}1 {û}{{\^u}}1
  {Â}{{\^A}}1 {Ê}{{\^E}}1 {Î}{{\^I}}1 {Ô}{{\^O}}1 {Û}{{\^U}}1
  {ã}{{\~a}}1 {ẽ}{{\~e}}1 {ĩ}{{\~i}}1 {õ}{{\~o}}1 {ũ}{{\~u}}1
  {Ã}{{\~A}}1 {Ẽ}{{\~E}}1 {Ĩ}{{\~I}}1 {Õ}{{\~O}}1 {Ũ}{{\~U}}1
  {œ}{{\oe}}1 {Œ}{{\OE}}1 {æ}{{\ae}}1 {Æ}{{\AE}}1 {ß}{{\ss}}1
  {ű}{{\H{u}}}1 {Ű}{{\H{U}}}1 {ő}{{\H{o}}}1 {Ő}{{\H{O}}}1
  {ç}{{\c c}}1 {Ç}{{\c C}}1 {ø}{{\o}}1 {å}{{\r a}}1 {Å}{{\r A}}1
  {€}{{\euro}}1 {£}{{\pounds}}1 {«}{{\guillemotleft}}1
  {»}{{\guillemotright}}1 {ñ}{{\~n}}1 {Ñ}{{\~N}}1 {¿}{{?`}}1 {¡}{{!`}}1 
}

\let\@makechapterheadorig\@makechapterhead
\def\afterchapter{asd}
\def\@makechapterhead#1{\@makechapterheadorig{#1}\afterchapter}


\endinput
