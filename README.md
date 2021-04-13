# PIX-BR

QR Code generator for the Brazilian payment system PIX
Tools for test [PIX QR Decoder](https://pix.nascent.com.br/tools/pix-qr-decoder/).

## Usage

```toml
[dependencies]
pix-br = "0.1.0"
```

## Example

**Parse String**

```rust
mod pix_br;
pub use crate::pix_br::pix;

fn main() {
    let brcode = pix::brcode(
        "460ef235-e006-492f-8686-eb5b8cfc5c82", //key (required)
        "Guilherme Luis Faustino",              //name (required)
        Some(82.82),                            //amount (optional)
        None,                                   //city (required)
        Some("Havaina e caipirinha"),           //description (optional)
        None,                                   //zip_code (optional)
        None,                                   //is_unique_transaction (optional)
    );

    pix::save_qrcode_png(brcode.clone(), "/path/file_name.png");

    pix::save_qrcode_svg(brcode.clone(), "/path/file_name.svg");

    pix::get_qrcode_data_uri(brcode.clone()); // return "data:image/png;base64, ... "

	pix::get_qrcode_svg(brcode); // return svg code
}

```

## Params 🎛️

### brcode()

`brcode(params)`:

| object key   | type    | required |
| ------------ | ------- | -------- |
| merchantKey  | string  | ✅       |
| merchantName | string  | ✅       |
| amount       | number  | ❌       |
| merchantCity | string  | ✅       |
| merchantCep  | string  | ❌       |
| description  | string  | ❌       |
| isUnique     | boolean | ❌       |

---

## Specification

### Latest revision version: 3.0.2 (2021-04-11)

### Specification by Bacen [(DOC)](https://www.bcb.gov.br/content/estabilidadefinanceira/forumpireunioes/AnexoI-PadroesParaIniciacaodoPix.pdf)

## Contributors ✨

Thanks goes to these wonderful people ([emoji key](https://allcontributors.org/docs/en/emoji-key)):

<table>
  <tr>
   <td align="center">
        <a href="https://github.com/iguilhermeluis">
        <img src="https://avatars.githubusercontent.com/u/26286830?v=3?s=100" width="100px;" alt=""/><br />
        <sub><b>Guilherme L. Faustino</b></sub></a><br />
        <a href="https://github.com/iguilhermeluis/pix-br/commits?author=iguilhermeluis" title="Code">💻</a>
        <a href="https://github.com/iguilhermeluis/pix-br/commits?author=iguilhermeluis" title="Documentation">📖</a>  
        <a href="https://github.com/iguilhermeluis/pix-br/commits?author=iguilhermeluis" title="Tests">⚠️</a>  
   </td>
  
    
  </tr>
</table>
