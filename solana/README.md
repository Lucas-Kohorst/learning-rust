### Solana Reading and Resources 
- [Anchor](https://project-serum.github.io/anchor/getting-started/installation.html#install-rust)
- [Learning how to build on solana](https://www.brianfriel.xyz/learning-how-to-build-on-solana/)
- [Understanding Program derived Addresses](https://www.brianfriel.xyz/understanding-program-derived-addresses/)
- [The complete solana development guide](https://dev.to/dabit3/the-complete-guide-to-full-stack-solana-development-with-react-anchor-rust-and-phantom-3291)
- [Solana Account model](https://solana.wiki/zh-cn/docs/account-model/)
- [Learning solana tweet thread](https://mobile.twitter.com/pencilflip/status/1451949960065335302)
- [Solana PDAs (again)](https://twitter.com/pencilflip/status/1455948263853600768?s=12&utm_source=pocket_mylist)
- [Debugging custom anchor messages](https://www.notion.so/Debugging-Custom-Anchor-Errors-b8540dd418c44a4e939ab17c56a3fd3b)
- [Solana Onboarding](https://github.com/ilmoi/solana-onboarding)
- [Solana Programming concepts](https://hackmd.io/@adamisrusty/HkVyZHBoO)
- [Programming on solana an introduction](https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/)
- [Anchor Basic-0 walkthrough](https://mirror.xyz/0x840B1dC2abb99f1F86D549303719610F346B2aaF/w3-WcRd8aablvFf8Er5qH4H4b-x-8UVfhElMv6Uwick)
- [Learning anchor](https://www.stasha.dev/girri/learning-anchor)
- [Simple vote program](https://github.com/bfriel/crunchy-vs-smooth-v2)
- [Solana Program Library](https://github.com/solana-labs/solana-program-library/)

- When creating a solana program you break the files into libraries and modules
  - Then in a `mod` of a program you split it into the "business logic" (all functions required) and the structs that define what interacts with your program (nice because this does all the validation for you)