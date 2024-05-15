# Riwayat Transaksi
Microservice dari repository **Game Time** yang memiliki peran sebagai pengatur riwayat transaksi antarpengguna aplikasi.

[![main](https://github.com/B9JagoNgadpro/gametime_riwayat/actions/workflows/rust.yml/badge.svg?branch=main)](https://github.com/B9JagoNgadpro/gametime_riwayat/actions/workflows/rust.yml)

## Monitoring
Untuk bagian monitoring, sebenarnya pada *deployment* Railway sudah disediakan visualisasi untuk memonitor penggunaan CPU, memory, dan network.

![Railway Monitoring](docs/railway-monitoring.png)

Saya juga sudah mencoba untuk melihat metrik menggunakan prometheus di sini.

![alt text](docs/prometheus-metrics.png)

## Profiling
Kurangnya tutorial menggunakan Rust membuat saya kesulitan dalam menerapkan profiling. Saya sudah mencoba untuk menggunakan package `hyperfine` untuk benchmarking dan `flamegraph` untuk visualisasi juga, namun keduanya tidak dapat dijalankan pada komputer saya sebab saya menggunakan windows. Untuk saat ini saya masih belum menemukan solusinya, namun akan saya perbaiki secepat mungkin.

![Flamegraph Error](docs/flamegraph-error.png)

## Lisensi