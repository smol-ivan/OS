#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#define MAX_ITEMS 50

typedef struct {
  int codigo;
  char marca[50];
  char modelo[50];
  float precio;
} Articulo;

int comparar(const void *a, const void *b);

int main() {
  FILE *archivo;
  Articulo articulos[MAX_ITEMS];
  int count = 0;

  archivo = fopen("data/papeleria.txt", "r");
  if (archivo == NULL) {
    printf("Error al abrir el archivo.\n");
    exit(1);
  }

  while (fscanf(archivo, "%d,%[^,],%[^,],%f\n", &articulos[count].codigo,
                articulos[count].marca, articulos[count].modelo,
                &articulos[count].precio) != EOF) {
    count++;
  }

  fclose(archivo);

  qsort(articulos, count, sizeof(Articulo), comparar);

  printf("%-10s %-20s %-35s %-10s\n", "Codigo", "Marca", "Modelo", "Precio");

  for (int i = 0; i < count; i++) {
    printf("%-10d %-20s %-35s %-10.2f\n", articulos[i].codigo,
           articulos[i].marca, articulos[i].modelo, articulos[i].precio);
  }

  return 0;
}

int comparar(const void *a, const void *b) {
  return ((Articulo *)a)->codigo - ((Articulo *)b)->codigo;
}