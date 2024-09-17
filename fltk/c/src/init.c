#include <errno.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/ipc.h>
#include <sys/shm.h>
#include <sys/types.h>

#define FILEKEY "/bin/cat"
#define MAX_BUF 1
#define MAX_FIELDS 5
#define MAX_STRING_LENGTH 50

void get_registros(char registros[][MAX_FIELDS][MAX_STRING_LENGTH],
                   int *count) {
  FILE *archivo;
  archivo = fopen("src/registros.txt", "r");
  if (archivo == NULL) {
    printf("Error al abrir el archivo.\n");
    exit(1);
  }

  // Ignorar la primera línea (headers)
  char buffer[256];
  fgets(buffer, sizeof(buffer), archivo);

  while (fscanf(archivo, "%49[^,],%49[^,],%49[^,],%49[^,],%49[^\n]\n",
                registros[*count][0], registros[*count][1],
                registros[*count][2], registros[*count][3],
                registros[*count][4]) != EOF) {
    (*count)++;
  }

  fclose(archivo);
}

void set_shared_memory(char registros[][MAX_FIELDS][MAX_STRING_LENGTH],
                       int count) {
  key_t key;
  int shmid;
  char(*shm)[MAX_FIELDS][MAX_STRING_LENGTH];

  key = ftok(FILEKEY, 0);
  if (key == -1) {
    perror("ftok");
    exit(1);
  }

  shmid = shmget(key, sizeof(char[MAX_FIELDS][MAX_STRING_LENGTH]) * count,
                 IPC_CREAT | 0777);
  if (shmid == -1) {
    perror("shmget");
    exit(1);
  }

  shm = shmat(shmid, NULL, 0);
  if (shm == (void *)-1) {
    perror("shmat");
    exit(1);
  }

  for (int i = 0; i < count; i++) {
    for (int j = 0; j < MAX_FIELDS; j++) {
      strcpy(shm[i][j], registros[i][j]);
    }
  }

  shmdt(shm);
}

int main() {
  char registros[100][MAX_FIELDS][MAX_STRING_LENGTH];
  int count = 0;

  get_registros(registros, &count);
  set_shared_memory(registros, count);

  return 0;
}