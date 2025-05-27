class VehicleSystem {
    constructor() {
        this.state = {
            engineOn: false,
            lightsOn: false,
            trunkOpen: false
        };
        
        this.initializeElements();
        this.setupEventListeners();
    }

    initializeElements() {
        // Status elements
        this.engineStatus = document.getElementById('engine-status');
        this.lightsStatus = document.getElementById('lights-status');
        this.trunkStatus = document.getElementById('trunk-status');
        
        // Buttons
        this.btnLigar = document.getElementById('btn-ligar');
        this.btnDesligar = document.getElementById('btn-desligar');
        this.btnFarol = document.getElementById('btn-farol');
        this.btnPortaMalas = document.getElementById('btn-porta-malas');
        this.btnBuzina = document.getElementById('btn-buzina');
        
        // Other elements
        this.honkDuration = document.getElementById('honk-duration');
        this.logMessages = document.getElementById('log-messages');
    }

    setupEventListeners() {
        this.btnLigar.addEventListener('click', () => this.turnOn());
        this.btnDesligar.addEventListener('click', () => this.turnOff());
        this.btnFarol.addEventListener('click', () => this.toggleLights());
        this.btnPortaMalas.addEventListener('click', () => this.toggleTrunk());
        this.btnBuzina.addEventListener('click', () => this.honk());
    }

    updateStatus(element, isOn, onText, offText) {
        element.textContent = isOn ? onText : offText;
        element.className = `status-value ${isOn ? 'on' : 'off'}`;
    }

    log(message) {
        const time = new Date().toLocaleTimeString();
        const logEntry = document.createElement('div');
        logEntry.className = 'log-message';
        logEntry.textContent = `[${time}] ${message}`;
        this.logMessages.appendChild(logEntry);
        this.logMessages.scrollTop = this.logMessages.scrollHeight;
    }

    turnOn() {
        if (!this.state.engineOn) {
            this.state.engineOn = true;
            this.updateStatus(this.engineStatus, true, 'Ligado', 'Desligado');
            this.btnLigar.disabled = true;
            this.btnDesligar.disabled = false;
            this.btnFarol.disabled = false;
            this.btnBuzina.disabled = false;
            this.log('Veículo ligado com sucesso');
        }
    }

    turnOff() {
        if (this.state.engineOn) {
            this.state.engineOn = false;
            this.state.lightsOn = false;
            this.updateStatus(this.engineStatus, false, 'Ligado', 'Desligado');
            this.updateStatus(this.lightsStatus, false, 'Acesos', 'Apagados');
            this.btnLigar.disabled = false;
            this.btnDesligar.disabled = true;
            this.btnFarol.disabled = true;
            this.btnBuzina.disabled = true;
            this.log('Veículo desligado com sucesso');
        }
    }

    toggleLights() {
        if (this.state.engineOn) {
            this.state.lightsOn = !this.state.lightsOn;
            this.updateStatus(this.lightsStatus, this.state.lightsOn, 'Acesos', 'Apagados');
            this.log(this.state.lightsOn ? 'Faróis acesos com sucesso' : 'Faróis apagados com sucesso');
            this.btnFarol.textContent = this.state.lightsOn ? 'Apagar Faróis' : 'Acender Faróis';
        }
    }

    toggleTrunk() {
        this.state.trunkOpen = !this.state.trunkOpen;
        this.updateStatus(this.trunkStatus, this.state.trunkOpen, 'Aberto', 'Fechado');
        this.log(this.state.trunkOpen ? 'Porta-malas aberto com sucesso' : 'Porta-malas fechado com sucesso');
        this.btnPortaMalas.textContent = this.state.trunkOpen ? 'Fechar Porta-malas' : 'Abrir Porta-malas';
    }

    async honk() {
        if (this.state.engineOn) {
            const duration = parseInt(this.honkDuration.value);
            if (duration >= 1 && duration <= 10) {
                this.btnBuzina.disabled = true;
                this.log(`Buzina acionada por ${duration} segundos`);
                await new Promise(resolve => setTimeout(resolve, duration * 1000));
                this.btnBuzina.disabled = false;
            } else {
                this.log('Erro: A duração da buzina deve ser entre 1 e 10 segundos');
            }
        }
    }
}

// Initialize the system when the page loads
document.addEventListener('DOMContentLoaded', () => {
    new VehicleSystem();
});