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

    pid_t pid;

    if (opcion == 'a')
    {
      pid = fork();

      if (pid == 0)
      {
        execlp("ls", "ls", "-l", "data", NULL);

        perror("Error al ejecutar ls");
        exit(1);
      }
      else if (pid > 0)
      {
        wait(&status);
      }
      else
      {
        perror("Error al crear proceso hijo");
        exit(1);
      }
    }
    else if (opcion == 'b' || opcion == 'c')
    {
      pid = fork();
      if (pid == 0)
      {
        if (opcion == 'b')
        {
          execl("./bin/leeArchivo", "leeArchivo", NULL);
        }
        else if (opcion == 'c')
        {
          execl("./bin/buscaRegistro", "buscaRegistro", NULL);
        }
      }
      else if (pid > 0)
      {
        wait(&status);
      }
      else
      {
        perror("Error al crear proceso hijo");
        exit(1);
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