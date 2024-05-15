onmessage = function (e) {
  const img = new Image()
  img.src = e.data
  img.onload = ()=>{
    postMessage(img.src)
  }
}
