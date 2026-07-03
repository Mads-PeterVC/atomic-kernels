# WASM Example

This is an example of the viewer running in WebAssembly. 

You can upload an `.xyz`-file, currently the file can only contain 1 structure, but that is 
a limitation of the file reader implementation not of the viewer.

<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <style>
      #viewer-root {
        width: 900px;
        height: 500px;
      }
      canvas {
        background-color: white;
        display: block;
        width: 100%;
        height: 100%;
      }
    </style>
  </head>
  <div>
    <input type="file" id="xyz" name="xyz"/>
  </div>
  <div id="viewer-root">
    <canvas id="viewer-canvas"></canvas>
  </div>

  <script type="module">
    import init, { WasmViewer } from './pkg/ak_wasm.js'
    await init()

    // const positions = new Float64Array([5.0, 5.0, 5.0, 6.0, 5.0, 5.0])
    // const numbers = new Int32Array([1, 8])
    // const cell = new Float64Array([10.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 10.0])

    const canvas = document.getElementById('viewer-canvas')
    const viewer = new WasmViewer(canvas);
    const xyzInput = document.getElementById('xyz')

    xyzInput.addEventListener('change', async () => {
      const file = xyzInput.files?.[0]
      if (!file) return

      try {
        viewer.load_xyz(await file.text())
      } catch (error) {
        console.error('Failed to load XYZ file', error)
      }
    })

    viewer.run()

    viewer.append_frame(new Float64Array([5.0, 5.0, 5.0, 6.0, 5.0, 5.0]), 
        new Int32Array([1, 8]), 
        new Float64Array([10.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 10.0]))

    viewer.append_frame(new Float64Array([5.0, 5.0, 5.0, 7.0, 5.0, 5.0]), 
        new Int32Array([1, 8]), 
        new Float64Array([10.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 10.0]))


  </script>
</html>


