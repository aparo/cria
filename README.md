# Cria - Local llama OpenAI-compatible API

The objective is to serve a local `llama-2` model by mimicking an OpenAI API service.
The llama2 model **runs on GPU** using `ggml-sys` crate with specific compilation flags.

## Quickstart:

1. Git clone project

   ```bash
   git clone git@github.com:AmineDiro/cria.git
   cd cria/
   git submodule update --init --recursive
   ```

2. Build project ( I ❤️ cargo !).

   ```bash
   cargo b --release
   ```

   - For `cuBLAS` (nvidia GPU ) acceleration use
     ```bash
     cargo b --release --features cublas
     ```
   - For `metal` acceleration use
     ```bash
     cargo b --release --features metal
     ```
     > ❗ NOTE: If you have issues building for GPU, checkout the building issues section

3. Download GGML `.bin` LLama-2 quantized model (for example [llama-2-7b](https://huggingface.co/TheBloke/Llama-2-7B-GGML/tree/main))
4. Run API, use the `use-gpu` flag to offload model layers to your GPU
   ```bash
   ./target/cria -a llama --model {MODEL_BIN_PATH} --use-gpu --gpu-layers 32
   ```
## Command line arguments reference
All the parameters can be passed as environment variables or command line arguments. Here is the reference for the command line arguments:

```bash
./target/cria --help

Usage: cria [OPTIONS]

Options:
  -a, --model-architecture <MODEL_ARCHITECTURE>      [default: llama]
      --model <MODEL_PATH>                           
  -v, --tokenizer-path <TOKENIZER_PATH>              
  -r, --tokenizer-repository <TOKENIZER_REPOSITORY>  
  -H, --host <HOST>                                  [default: 0.0.0.0]
  -p, --port <PORT>                                  [default: 3000]
  -m, --prefer-mmap                                  
  -c, --context-size <CONTEXT_SIZE>                  [default: 2048]
  -l, --lora-adapters <LORA_ADAPTERS>                
  -u, --use-gpu                                      
  -g, --gpu-layers <GPU_LAYERS>                      
  -z, --zipkin-endpoint <ZIPKIN_ENDPOINT>            
  -h, --help                                         Print help
```

For environment variables, just prefix the argument with `CRIA_` and use uppercase letters. For example, to set the model path, you can use `CRIA_MODEL` environment variable.

There is a an example .env.sample file in the project root directory.

# Prometheus Metrics

We are exporting Prometheus metrics via the `/metrics` endpoint. 

# Tracing
We are tracing performance metrics using `tracing` and `tracing-opentelemetry` crates.

You can use the `--zipkin-endpoint` to export metrics to a zipkin endpoint.

There is a docker-compose file in the project root directory to run a local zipkin server on port `9411`.

<div align="center">
<img src="./zipkin_screenshot.png"  alt="screenshot"/>
</div>


# Completion Example

You can use `openai` python client or directly use the `sseclient` python library and stream messages.
Here is an example :

<details><summary>Here is a example using a Python client</summary>

```python
import json
import sys
import time

import sseclient
import urllib3

url = "http://localhost:3000/v1/completions"


http = urllib3.PoolManager()
response = http.request(
    "POST",
    url,
    preload_content=False,
    headers={
        "Content-Type": "application/json",
    },
    body=json.dumps(
        {
            "prompt": "Morocco is a beautiful country situated in north africa.",
            "temperature": 0.1,
        }
    ),
)

client = sseclient.SSEClient(response)

s = time.perf_counter()
for event in client.events():
    chunk = json.loads(event.data)
    sys.stdout.write(chunk["choices"][0]["text"])
    sys.stdout.flush()
e = time.perf_counter()

print(f"\nGeneration from completion took {e-s:.2f} !")

```

</details>

You can clearly see generation using my M1 GPU:

<p align="center">
<img src="contents/../content/generation.gif" width=1000px height=auto />
</p>

<!-- Here is the llama-2 response:

```ipython
In [8]: %run test_sse.py
nobody knows how many people live there, but it's estimated that the population is around 3
0 million.
The Moroccans are very friendly and welcoming people. They love to meet foreigners and they will be happy if you speak their language (Arabic).
Morocco is a Muslim country so don't expect to see any women wearing bikinis on the beach or at the pool. You can find some of them in Marrakech though!
If you want to visit Morocco, I recommend you to go during spring or autumn because summer is too hot and winter is cold.
I hope you enjoy your stay in this beautiful country!

Generation from completion took 2.25 !
``` -->

## Building with GPU issues

I had some issues compiling `llm` crate with `cuda` support for my RTX2070 Super (Turing architecture). After some debugging, I needed to provide nvcc with the correct gpu-architecture version, for now `ggml-sys` crates only supports. Here are the set of changes to the `build.rs` :

```diff
diff --git a/crates/ggml/sys/build.rs b/crates/ggml/sys/build.rs
index 3a6e841..ef1e1b0 100644
--- a/crates/ggml/sys/build.rs
+++ b/crates/ggml/sys/build.rs
@@ -330,8 +330,9 @@ fn enable_cublas(build: &mut cc::Build, out_dir: &Path) {
             .arg("--compile")
             .arg("-cudart")
             .arg("static")
-            .arg("--generate-code=arch=compute_52,code=[compute_52,sm_52]")
-            .arg("--generate-code=arch=compute_61,code=[compute_61,sm_61]")
+            .arg("--generate-code=arch=compute_75,code=[compute_75,sm_75]")
             .arg("-D_WINDOWS")
             .arg("-DNDEBUG")
             .arg("-DGGML_USE_CUBLAS")
@@ -361,8 +362,7 @@ fn enable_cublas(build: &mut cc::Build, out_dir: &Path) {
             .arg("-Illama-cpp/include/ggml")
             .arg("-mtune=native")
             .arg("-pthread")
-            .arg("--generate-code=arch=compute_52,code=[compute_52,sm_52]")
-            .arg("--generate-code=arch=compute_61,code=[compute_61,sm_61]")
+            .arg("--generate-code=arch=compute_75,code=[compute_75,sm_75]")
             .arg("-DGGML_USE_CUBLAS")
             .arg("-I/usr/local/cuda/include")
             .arg("-I/opt/cuda/include")
```

The only thing left to do is to change `Cargo.toml` file to

## TODO/ Roadmap:

- [x] Run Llama.cpp on CPU using llm-chain
- [x] Run Llama.cpp on GPU using llm-chain
- [x] Implement `/models` route
- [x] Implement basic `/completions` route
- [x] Implement streaming completions SSE
- [x] Cleanup cargo features with llm
- [x] Support MacOS Metal
- [x] Merge completions / completion_streaming routes in same endpoint
- [x] Implement `/embeddings` route
- [x] Implement route `/chat/completions`
- [ ] Better errors
- [X] Setup good tracing
- [ ] Implement streaming chat completions SSE
- [X] Metrics ??
- [ ] Batching requests(ala iouring):
  - For each response put an entry in a ringbuffer queue with : Entry(Flume mpsc (resp_rx,resp_tx))
  - Spawn a model in separate task reading from ringbuffer, get entry and put each token in response
  - Construct stream from flue resp_rx chan and return SSE(stream) to user.

## Routes

- Checkout : https://platform.openai.com/docs/api-reference/
