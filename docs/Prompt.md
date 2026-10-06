
## Exemplo de uso temporario

```rust
use aedii_sgpme::prompts;
use strum::{EnumIter, EnumMessage};

#[derive(Debug, Clone, Copy, EnumIter, EnumMessage)]
enum Acao {
	#[strum(message = "Criar um novo registro")]
	Create,
	#[strum(message = "Listar registros")]
	List,
	#[strum(message = "Sair")]
	Exit,
}

// Para usar as funções você PRECISA definir o tipo de variavel com o : Tipo (: usize), caso

// contrario a função não sabe para qual tipo deve fazer o casting do input

fn main() {
	println!("Hello World!");
	let _r: usize = prompts::prompt("Digite um numero qualquer");
	let _action: Acao = prompts::choice_prompt("Qual ação deseja fazer?");
}
```

<iframe frameborder="0" scrolling="no" style="width:100%; height:1737px;" allow="clipboard-write" src="https://emgithub.com/iframe.html?target=https%3A%2F%2Fgithub.com%2FJaquesBoeno%2Faedii-sgpme%2Fblob%2Fmain%2Fsrc%2Fprompts.rs&style=default&type=code&showBorder=on&showLineNumbers=on&showFileMeta=on&showFullPath=on&showCopy=on"></iframe>
