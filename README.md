# 🚗 Vehicle Automation System

Este projeto nasceu como uma iniciativa acadêmica, mas foi desenvolvido com uma abordagem mais próxima do mercado: simular um sistema de **automação veicular** com foco em organização, integração entre camadas e tomada de decisão baseada em eventos.

A ideia foi ir além de “fazer funcionar” e pensar em **como esse tipo de sistema seria estruturado na prática**.

---

## 📌 Sobre o projeto

O sistema simula o controle de funcionalidades de um veículo de forma automatizada, permitindo monitoramento e execução de ações em tempo real.

Durante o desenvolvimento, busquei trabalhar não só a implementação, mas também a **separação de responsabilidades**, comunicação entre partes do sistema e clareza na lógica.

---

## 🎯 Objetivo

Mais do que apenas entregar funcionalidades, o objetivo foi:

- Explorar automação aplicada a um contexto real  
- Praticar integração entre backend e interface  
- Trabalhar organização de código e arquitetura  
- Simular cenários próximos de sistemas embarcados  

---

## ⚙️ Funcionalidades

- 🔐 Simulação de ignição e controle de acesso  
- 💡 Controle de luzes  
- 📊 Monitoramento de status em tempo real  
- 🔄 Execução de ações automatizadas  
- 🧠 Lógica orientada a eventos  

---

## 🛠️ Tecnologias utilizadas

### Backend
- Rust → foco em performance, controle e previsibilidade

### Frontend
- HTML5  
- CSS3  
- JavaScript  
- TypeScript  

---

## 🧩 Arquitetura

O projeto foi estruturado pensando em separação clara de responsabilidades:

- **Backend (Rust):** centraliza a lógica da automação e controle do sistema  
- **Frontend:** responsável pela interação e visualização  
- **Comunicação:** integração entre interface e lógica principal  

A ideia foi manter o sistema simples, mas já seguindo um padrão que permita evolução.

---

### 🔧 Backend (Rust)

```bash
cd backend
cargo run
