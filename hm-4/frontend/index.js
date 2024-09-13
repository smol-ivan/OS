fetch('http://127.0.0.1:1234/articulos')
  .then(res => res.json())
  .then(articulos => {
    const html = articulos.map(articulo => {
      return `
      <tr>
        <td>${articulo.numero}</td>
        <td>${articulo.producto}</td>
        <td>${articulo.precio}</td>
      </tr>
      `
    }).join('')

    document.querySelector('#tabla-articulos tbody').innerHTML = html
  })