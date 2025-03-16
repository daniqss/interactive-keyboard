# Modelo de memoria de Traballo Fin de Grao

Este proxecto LaTeX constitúe un modelo de referencia para as memorias de Traballo Fin de Grao
do **Grao en Enxeñaría Informática** e do **Grao en Ciencia e Enxeñaría de Datos**
da Facultade de Informática da Universidade da Coruña.

Antes de usar este modelo, por favor, revisa con atención as [`FAQ (Frequently Asked Questions)`](https://gitlab.com/lauramcastro/modelo-tfg-gei-fic/-/wikis/Frequently-asked-questions-(faq)).

## Estrutura

  1) Ficheiros de autoría, contribucións, licenza e atribución

     > `AUTHOR`
     >
     > `CONTRIBUTING.md`
     >
     > `COPYING`
     >
     > `CREDITS`

  2) Ficheiro de estilo: `estilo_tfg.sty`

  3) Ficheiro principal: `memoria_tfg.tex`

  4) Directorios:

     > `anexos/`		Contén os capítulos con materiais adicionais.
     >
     > `bibliografia/`	Contén a bibliografía e outros posibles índices (termos, glosario).
     >
     > `contido/`		Contén os capítulos da memoria.
     >
     > `imaxes/`		Contén as imaxes da memoria.
     >
     > `portada/`		Contén a portada, resumo e palabras chave.

## Xeración da versión PDF

A versión PDF pódese xerar empregando a ferramenta `latexmk`, que asegura o correcto procesamento
de índices, bibliografía e referencias:

     latexmk -xelatex memoria_tfg.tex

A ferramenta `latexmk` pódese empregar de xeito que monitorice o proxecto e recompile automaticamente
a memoria en caso de producirse cambios nos diferentes ficheiros que a conforman:

     latexmk -xelatex -pvc memoria_tfg.tex

## Eliminación dos ficheiros auxiliares

No momento no que a redacción da memoria do TFG se dea por finalizada, os ficheiros auxiliares xerados poden eliminarse doadamente con:

     latexmk -xelatex -c

Non se recomenda a eliminación dos ficheiros auxiliares durante o proceso de redacción, xa que a súa presenza contribúe a axilizar o proceso de compilación e xeración do PDF en construción.
