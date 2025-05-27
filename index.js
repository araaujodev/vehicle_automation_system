// Sistema de Automação Veicular
const readline = require('readline');

const rl = readline.createInterface({
  input: process.stdin,
  output: process.stdout
});

console.log("\n=== Sistema de Automação Veicular ===");
console.log("Digite comandos para controlar o veículo.");
console.log("Digite 'exit' para sair.\n");
console.log("Comandos disponíveis:");
console.log("  - ligar (liga o veículo)");
console.log("  - desligar (desliga o veículo)");
console.log("  - acender_farol (acende os faróis)");
console.log("  - apagar_farol (apaga os faróis)");
console.log("  - abrir_porta_malas (abre o porta-malas)");
console.log("  - buzinar [segundos] (aciona a buzina)\n");

function prompt() {
  rl.question('> ', (command) => {
    if (command.trim() === 'exit') {
      console.log('Desligando o sistema...');
      rl.close();
      return;
    }
    
    handleCommand(command);
    prompt();
  });
}

function handleCommand(command) {
  switch(command.trim()) {
    case 'ligar':
      console.log('Veículo ligado com sucesso');
      break;
    case 'desligar':
      console.log('Veículo desligado com sucesso');
      break;
    case 'acender_farol':
      console.log('Faróis acesos com sucesso');
      break;
    case 'apagar_farol':
      console.log('Faróis apagados com sucesso');
      break;
    case 'abrir_porta_malas':
      console.log('Porta-malas aberto com sucesso');
      break;
    default:
      if (command.startsWith('buzinar')) {
        const parts = command.split(' ');
        const seconds = parts[1] ? parseInt(parts[1]) : 1;
        if (seconds > 0 && seconds <= 10) {
          console.log(`Buzina acionada por ${seconds} segundos`);
        } else {
          console.log('Erro: A duração da buzina deve ser entre 1 e 10 segundos');
        }
      } else {
        console.log('Comando inválido');
      }
  }
}

prompt();