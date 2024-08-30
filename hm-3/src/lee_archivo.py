from typing import List
import subprocess
import tempfile
import time

def main()->None:
    try:
        with open('data/registros.txt', 'r') as archivo:
            print("Leyendo registros...")
            registros:List[str] = archivo.readlines()
    except Exception as e:
        print("Error al leer el archivo de registros")
        print(e)
    else:
        header = registros[0].split(',')
        registros = registros[1:]
        registros = [registro.split(',') for registro in registros]
        registros = sorted(registros, key=lambda x: x[0])

        # Preparar archivo temporal para mostrar registros en pantalla con less
        with tempfile.NamedTemporaryFile(delete=False, mode='w', suffix=".txt") as temp_file:
            temp_file.write(f'{header[0]:<15}{header[1]:<15}{header[2]:<15}{header[3]:<15}{header[4]:<15}\n')

            for registro in registros:
                temp_file.write(f'{registro[0]:<15}{registro[1]:<15}{registro[2]:<15}{registro[3]:<15}{registro[4]:<15}\n')
        
        # Aviso de lectura con cmd less
        print("Se abrirá el archivo de registros en pantalla con 'less', presione 'q' para salir")
        time.sleep(3)
        subprocess.run(["less", temp_file.name])
        
if __name__ == '__main__':
    main()