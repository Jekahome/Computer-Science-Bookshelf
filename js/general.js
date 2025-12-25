
document.addEventListener('DOMContentLoaded', () => {
    document.documentElement.style.setProperty('--content-max-width', '70%');
});

//--------------------------------------------------------------------------------
(() => {

document.addEventListener('click', (e) => {
    if (e.target.classList.contains('sim-fullscreen-btn')) {
        const circuitId = e.target.dataset.circuitId;
        
        // Находим wrapper по data-circuit-id
        const wrapper = document.querySelector(`.sim-wrapper[data-circuit-id="${circuitId}"]`);
        
        if (wrapper) {
            // Переключаем fullscreen только для этого wrapper
            wrapper.classList.toggle("sim-fullscreen");
            
            // Находим iframe внутри этого wrapper
            //const iframe = wrapper.querySelector('.sim-iframe');    
        }
    }
});

/*
  const iframe = document.getElementById('sim-iframe');

  const circuit = `$ 1 0.000005 10.20027730826997 50 5 50 5e-11
r 256 176 256 304 0 500
172 304 176 304 128 0 7 5 5 0 0 0.5 Voltage
g 256 336 256 352 0 0
w 256 304 256 336 1
r 352 176 352 304 0 1000
w 352 304 352 336 1
g 352 336 352 352 0 0
w 304 176 352 176 0
w 256 176 304 176 0
  `;

  // Заменяем $ на %24 для URL
  const encodedCircuit = circuit.replace(/\$/g, '%24').replace(/\n/g, '%0A');
  iframe.src = `./static/circuit-frame.html?running=0&cct=${encodedCircuit}`;
*/

})();