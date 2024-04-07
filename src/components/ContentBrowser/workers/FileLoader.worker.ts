onmessage = function (e) {
  console.log(e.data);
  const img = new Image()
  img.src = e.data
  img.onload = ()=>{
    postMessage(img.src)
  }
}
