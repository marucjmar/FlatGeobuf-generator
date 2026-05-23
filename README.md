# FlatGeobuf GeoJSON Converter (WASM)

A lightweight tool for converting **GeoJSON files into FlatGeobuf (FGB)** directly in the browser.

---

👉 Try it here:  
https://marucjmar.github.io/flatbuf-generator/

## 🚀 What is this?

This project is a **browser-based GeoJSON → FlatGeobuf converter**.

It allows you to upload a GeoJSON file and convert it into the highly efficient FlatGeobuf binary format — without any backend or server-side processing.

---

## 🌐 100% Browser-Based

All processing happens entirely in the **browser**:

- No server required  
- No file uploads to external services  
- Fully offline-capable once loaded  

Your geospatial data never leaves your machine.

---

## 🦀 Powered by Rust + WebAssembly

This tool is built on top of the official **FlatGeobuf Rust implementation** and compiled to WebAssembly.

Original library:
https://github.com/flatgeobuf/flatgeobuf :contentReference[oaicite:0]{index=0}

The Rust core handles:
- Fast GeoJSON parsing
- Efficient FlatGeobuf encoding
- Spatial indexing (R-tree)
- Binary serialization

It is compiled to WASM to run directly inside the browser at near-native performance.

---

## ⚡ Features

- Convert GeoJSON → FlatGeobuf
- Runs entirely in the browser
- No backend required
- Fast WASM-based processing
- Supports large datasets
- Based on the official FlatGeobuf spec

---

## 🧪 Use Case

This tool is useful for:

- GIS developers
- Geospatial data pipelines
- Offline data processing
- Web mapping applications
- Vector tile preprocessing

---

## 🔗 Live Demo

👉 Try it here:  
https://marucjmar.github.io/flatbuf-generator/

---

## 📦 Tech Stack

- Rust
- WebAssembly (wasm-pack)
- FlatGeobuf
- Vite
- TypeScript

---

## 📄 License

This project is built on top of the FlatGeobuf ecosystem and follows its licensing model.

---

## 💡 Notes

FlatGeobuf is a high-performance binary format designed for fast reading, streaming, and spatial indexing of geospatial vector data.

It is significantly more efficient than traditional formats like GeoJSON for large datasets.
