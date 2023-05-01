Leak Seek is inspired by another popular tool called [Gitleaks](https://github.com/gitleaks/gitleaks), which also scans Git repositories for hardcoded secrets. However, Leak Seek offers several additional features and improvements over [Gitleaks](https://github.com/gitleaks/gitleaks) like memory safety (Feature Of Rust).

Leak Seek is written in Rust, which is a modern, high-performance programming language that has gained popularity in recent years due to its speed, memory safety, and reliability. Rust is well-suited for systems programming and is ideal for developing software that requires high levels of concurrency, parallelism, and low-level control over hardware resources.

Overall, Leak Seek is a powerful tool for detecting and preventing hardcoded secrets in Git repositories, and can help organizations improve their overall security posture and reduce the risk of data breaches and other security incidents.

## Getting Started

Leak Seek can be installed using Rust. For now 

Currently Leak Seek requires users to compile the tool themselves, as precompiled binaries are not yet available. However, this may change in future iterations of the tool, as the development team continues to refine and enhance its capabilities.

To Compile Use The Following Code:

``` cargo build --release ```

After you have compiled You can find the binary at target/release

### Commands

#### For Git

``` ./leakseek -g detect <PATH> ``` 

#### For File systems

``` ./leakseek detect <PATH> ```

