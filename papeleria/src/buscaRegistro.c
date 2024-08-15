#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct
{
  int codigo;
  char marca[50];
  char modelo[50];
  float precio;
} Articulo;

int main()
{
  FILE *archivo;
  Articulo articulo;
  int codigo_buscado;
  int encontrado = 0;

  printf("Ingrese el código a buscar: ");
  scanf("%d", &codigo_buscado);

  archivo = fopen("data/papeleria.txt", "r");
  if (archivo == NULL)
  {
    printf("Error al abrir el archivo.\n");
    exit(1);
  }

  while (fscanf(archivo, "%d,%[^,],%[^,],%f\n", &articulo.codigo, articulo.marca, articulo.modelo, &articulo.precio) != EOF)
  {
    if (articulo.codigo == codigo_buscado)
    {
      printf("%-10s %-20s %-35s %-10s\n", "Codigo", "Marca", "Modelo", "Precio");
      printf("%-10d %-20s %-35s %-10.2f\n", articulo.codigo, articulo.marca, articulo.modelo, articulo.precio);

      encontrado = 1;
      break;
    }
  }

  if (!encontrado)
  {
    printf("Artículo con código %d no encontrado.\n", codigo_buscado);
  }

  fclose(archivo);

  return 0;
}
