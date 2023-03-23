# Task Report

Script hacer un reporte de las tareas realizadas en el día.

## Datasource
Cada día registro en notas el ticket que estoy trabajando y el ticket anterior para informar en la daily del equipo.
Por el momento la nota es creada a mano diariamente.
La estructura de la nota es:
```
Daily:
	* MBM-1234 Tarea de ayer [x]
Today:
	* MBM-1235 Nueva tarea []
```
El nombre del archivo contiene la fecha:
`20230323_notes.md`

## Report structure
Formato: impreso en pantalla por ahora
El reporte esperado tiene que ser así:
```
23/03/23
	- MBM-1010
	- MBM-1111
	- MBM-1123
24/03/23
	- MBM-8989
	- MBM-2222
27/03/23
	- MBM-2222
	- MBM-1111
```
