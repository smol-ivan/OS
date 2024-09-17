API_LINK = 'http://192.168.3.213:1234/articulos'

returnArticulos(API_LINK)

function returnArticulos(url) {
  fetch(url).then(res => res.json())
    .then(data => {
      const articulos = data.map(articulo => {
        return `
        <tr>
          <td>${articulo.numero}</td>
          <td>${articulo.producto}</td>
          <td>${articulo.precio}</td>
        </tr>
        `
      }).join('')

      document.querySelector('#tabla-articulos tbody').innerHTML = articulos
    })
}

document.getElementById('send_button').addEventListener('click', () => {
  const articuloId = document.getElementById('input_text').value;
  if (articuloId) {
    // Redirigir a la página del artículo usando params
    window.location.href = 'articulo.html?no_articulo=' + articuloId + '&accion=reservar';
  } else {
    alert('Por favor, ingrese un número de artículo válido.');
  }
});
