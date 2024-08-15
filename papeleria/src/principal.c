#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/wait.h>
#include <sys/types.h>

void mostrar_menu();

int main()
{
  char opcion;
  int status;

  while (1)
  {
    mostrar_menu();
    scanf(" %c", &opcion);

    if (opcion == 'a')
    {
      system("ls -l");
    }
    else if (opcion == 'b' || opcion == 'c')
    {
      pid_t pid = fork();
      if (pid == 0)
      {
        if (opcion == 'b')
        {
          execl("./leeArchivo", "leeArchivo", NULL);
        }
        else if (opcion == 'c')
        {
          execl("./buscaRegistro", "buscaRegistro", NULL);
        }
      }
      else
      {
        wait(&status);
      }
    }
    else if (opcion == 'd')
    {
      exit(0);
    }
    else
    {
      printf("Opción no válida\n");
    }
  }
  return 0;
}

void mostrar_menu()
{
  printf("----------- Negocio X ---------------\n");
  printf("a) Mostrar carpeta donde se encuentra el archivo a leer\n");
  printf("b) Leer el archivo del negocio X\n");
  printf("c) Buscar por clave en el archivo del negocio X\n");
  printf("d) Salir\n");
}