# vectores_rust
Iutepal

# Programa de cálculo de puntajes

Este programa fue creado por los estudiantes Fermín Chirinos y Ricardo Chávez. El programa lee las calificaciones de un archivo de texto, calcula el puntaje final para cada participante y escribe los resultados en otro archivo de texto.

## Cómo funciona

El programa se basa en el lenguaje de programación Rust y utiliza la biblioteca estándar para leer y escribir archivos.

El archivo de entrada se llama "Calificacion.txt" y debe contener las calificaciones de los participantes en el siguiente formato:

```
Nombre del participante, calificación1, calificación2, ...
```

El programa lee este archivo línea por línea. Para cada línea, el programa divide la línea en el nombre del participante y sus calificaciones. Luego, calcula el puntaje final para el participante y guarda el resultado.

El cálculo del puntaje final se realiza de la siguiente manera:

1. Las calificaciones se ordenan de menor a mayor.
2. Se eliminan la calificación más baja y la más alta.
3. Se calcula el promedio de las calificaciones restantes.
4. El promedio se multiplica por 

## Ramas
En el repositorio hay dos ramas, la rama "primera version" y la rama "main", la primera version calcula un promedio total de todos los puntajes obtenidos
de "Calificacion.txt" mientras que la versional final main, calcula el promedio de las puntuacion dividiendolas en grupos de dos, siendo asi varios promedios
