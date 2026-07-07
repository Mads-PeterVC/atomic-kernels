const WASM_MODULE_URL = "https://atomic-kernels.mads-peter.com/pkg/ak_wasm.js";

function viewerSrcdoc() {
  return `<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8">
    <style>
      html,
      body {
        width: 100%;
        height: 100%;
        margin: 0;
        overflow: hidden;
      }

      body {
        background: white;
      }

      canvas {
        display: block;
        width: 100%;
        height: 100%;
      }

      .error {
        box-sizing: border-box;
        width: 100%;
        height: 100%;
        display: flex;
        align-items: center;
        justify-content: center;
        padding: 16px;
        color: #8a1f11;
        background: #fff5f2;
        font: 14px/1.4 system-ui, -apple-system, BlinkMacSystemFont, "Segoe UI", sans-serif;
      }
    </style>
  </head>
  <body>
    <canvas id="viewer-canvas"></canvas>
    <script type="module">
      import init, { WasmViewer } from "${WASM_MODULE_URL}";

      let viewer = null;

      function showError(message, error) {
        if (error) {
          console.error(message, error);
        } else {
          console.error(message);
        }
        document.body.innerHTML = '<div class="error">' + message + '</div>';
      }

      function appendFrames(frames) {
        if (!viewer) {
          throw new Error("viewer is not ready");
        }
        if (!Array.isArray(frames) || frames.length === 0) {
          throw new Error("frame payload must contain at least one frame");
        }

        for (const frame of frames) {
          viewer.append_frame(
            new Float64Array(frame.positions),
            new Int32Array(frame.numbers),
            new Float64Array(frame.cell)
          );
        }
      }

      window.addEventListener("message", (event) => {
        if (event.source !== window.parent) {
          return;
        }

        const message = event.data;
        if (!message || message.type !== "ak-viewer-frames") {
          return;
        }

        try {
          appendFrames(message.frames);
        } catch (error) {
          showError("Failed to load Atomic Kernels viewer frames.", error);
        }
      });

      try {
        await init();

        const canvas = document.getElementById("viewer-canvas");
        viewer = new WasmViewer(canvas);
        viewer.run();
        window.parent.postMessage({ type: "ak-viewer-ready" }, "*");
      } catch (error) {
        showError("Failed to start Atomic Kernels viewer.", error);
      }
    </script>
  </body>
</html>`;
}

function render({ model, el }) {
  el.replaceChildren();

  const iframe = document.createElement("iframe");
  iframe.allow = "webgpu; fullscreen";
  iframe.srcdoc = viewerSrcdoc();
  iframe.style.width = "100%";
  iframe.style.height = "500px";
  iframe.style.border = "0";
  iframe.style.display = "block";

  const handleMessage = (event) => {
    if (event.source !== iframe.contentWindow) {
      return;
    }

    const message = event.data;
    if (!message || message.type !== "ak-viewer-ready") {
      return;
    }

    iframe.contentWindow.postMessage(
      {
        type: "ak-viewer-frames",
        frames: model.get("frames"),
      },
      "*"
    );
  };

  window.addEventListener("message", handleMessage);
  el.appendChild(iframe);

  return () => {
    window.removeEventListener("message", handleMessage);
    iframe.remove();
  };
}

export default { render };
