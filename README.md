<div align="center">
  <picture>
    <img alt="RIPGUARD: Integrasi Edge Intelligence dan Cloud Computing untuk Deteksi serta Pemetaan Rip Current di Kawasan Pantai Parangtritis, Bantul, Yogyakarta"
         src="https://raw.githubusercontent.com/KeyouXZ/RIPGUARD/refs/heads/main/RIPGUARD.svg"
         width="30%">
  </picture>

  <h2>Android Client • Rust Backend • YOLOv8 Training</h2>
  <p>Integrasi Edge Intelligence dan Cloud Computing untuk Deteksi serta Pemetaan Rip Current di Kawasan Pantai Parangtritis, Bantul, Yogyakarta</p>

  ![License](https://img.shields.io/badge/License-AGPLv3-blue.svg)
  [![Android](https://github.com/KeyouXZ/RIPGUARD/actions/workflows/android.yml/badge.svg)](https://github.com/KeyouXZ/RIPGUARD/actions/workflows/android.yml)
  [![Server](https://github.com/KeyouXZ/RIPGUARD/actions/workflows/server.yml/badge.svg)](https://github.com/KeyouXZ/RIPGUARD/actions/workflows/server.yml)
</div>

> RIPGUARD is an AI-powered early warning platform designed to detect **rip currents** from coastal imagery and deliver real-time hazard information through an Android application. Rip currents are narrow, fast-moving channels of water that flow away from the shore and are responsible for numerous coastal accidents worldwide. Because they are often difficult for beach visitors to recognize, traditional mitigation methods such as static warning signs and manual patrols may not provide sufficient protection. RIPGUARD addresses this challenge by combining a **YOLOv8-based computer vision model**, a **high-performance Rust backend**, and a **modern Android application built with Kotlin and Jetpack Compose**. The system performs real-time rip current detection, processes predictions on the server, and presents hazard information through an intuitive mobile interface. The project was developed as part of a research initiative focusing on disaster mitigation at Parangtritis Beach, Indonesia. Its goal is to improve public awareness, support safer tourism, and demonstrate how artificial intelligence can enhance coastal safety through accessible early warning technology.


# Why RIPGUARD?
Every year, rip currents contribute to preventable drowning incidents because they are nearly invisible to the untrained eye. While warning signs can inform visitors about general risks, they cannot reflect rapidly changing sea conditions.
RIPGUARD aims to bridge this gap by providing:
- Real-time rip current detection using deep learning
- Interactive visualization of hazardous coastal zones
- Mobile-first access for beach visitors
- A scalable backend designed with Rust for performance and reliability
- An end-to-end workflow covering AI training, inference, and mobile deployment

# Backend Workspace
| Crate            | Responsibility        |
| ---------------- | --------------------- |
| ripguard-handler | HTTP handlers         |
| ripguard-route   | API routing           |
| ripguard-service | Business logic        |
| ripguard-model   | Shared models         |
| ripguard-config  | Configuration         |
| ripguard-logger  | Logging               |
| ripguard-cli     | CLI utilities         |
| ripguard-adapter | External integrations |

## Roadmap
- [x] Android client
- [x] Rust API
- [x] Image detection
- [x] Report endpoint
- [x] ONNX Inference
- [ ] Calculate latitude and longitude
- [ ] Metrics
- [ ] Admin dashboard
