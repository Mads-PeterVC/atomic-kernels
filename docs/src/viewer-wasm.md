# WASM Example

This is an example of the viewer running in WebAssembly

!!! Example

  <html lang="en">
    <script type="module">
      import init, { WasmViewer } from './pkg/ak_wasm.js'
      await init()

      // const positions = new Float64Array([5.0, 5.0, 5.0, 6.0, 5.0, 5.0])
      // const numbers = new Int32Array([1, 8])
      // const cell = new Float64Array([10.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 10.0])

      const viewer = new WasmViewer();

      viewer.run()

      viewer.append_frame(new Float64Array([5.0, 5.0, 5.0, 6.0, 5.0, 5.0]), 
          new Int32Array([1, 8]), 
          new Float64Array([10.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 10.0]))

      viewer.append_frame(new Float64Array([5.0, 5.0, 5.0, 7.0, 5.0, 5.0]), 
          new Int32Array([1, 8]), 
          new Float64Array([10.0, 0.0, 0.0, 0.0, 10.0, 0.0, 0.0, 0.0, 10.0]))


    </script>
  </html>

