const API_LINK = 'http://192.168.3.213:1234/comprar';
document.addEventListener('DOMContentLoaded', () => {

  // Extraer el parámetro 'id' de la URL
  const urlParams = new URLSearchParams(window.location.search);
  const articuloId = urlParams.get('no_articulo');
  const accion = urlParams.get('accion'); // Obtener la acción de la URL

  if (articuloId && accion) {
    fetch(`${API_LINK}?no_articulo=${articuloId}&accion=${accion}`, {
      method: 'GET'
    })
      .then(response => {
        console.log(response);
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        const detalleDiv = document.getElementById('articulo-detalle');
        if (accion === 'reservar') {
          detalleDiv.innerHTML = `
            <p>Producto reservado:</p>
            <p>Identificador: ${data.identificador}</p>
            <p>Producto: ${data.producto}</p>
            <p>Marca: ${data.marca}</p>
            <p>Precio: ${data.precio}</p>
            <p>Cantidad: ${data.cantidad}</p>
          `;
        } else if (accion === 'confirmar') {
          detalleDiv.innerHTML = `
            <p>Compra confirmada:</p>
            <p>Mensaje: ${data}</p>
          `;
        }
      })
      .catch(error => {
        console.error('Error:', error);
        document.getElementById('articulo-detalle').innerText = 'Error al procesar la solicitud';
      });
  } else {
    document.getElementById('articulo-detalle').innerText = 'ID de artículo o acción no proporcionados';
  }
});