# Глава 2. Построение абстракций с помощью данных


<script>
    (function() {
        const localMap = {
            // JS скрипты
            'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.43.0/mode/scheme/scheme.min.js': 'sicp/js/scheme.min.js',
            'https://viebel.github.io/klipse/repo/js/biwascheme-0.6.6-min.js': 'sicp/js/biwascheme-0.6.6-min.js',
            
            // CSS
            'https://storage.googleapis.com/app.klipse.tech/css/codemirror.css': 'sicp/css/codemirror.css',
        };
        
        // Перехватываем XMLHttpRequest
        const XHR = XMLHttpRequest;
        const originalOpen = XHR.prototype.open;
        
        XHR.prototype.open = function(method, url, async, user, pass) {
            //console.log('XHR:', url);
            if (localMap[url]) {
                console.log(`Перехват XHR: ${url} → ${localMap[url]}`);
                return originalOpen.call(this, method, localMap[url], async, user, pass);
            }
            return originalOpen.call(this, method, url, async, user, pass);
        };
    
        console.log('Перехватчик установлен');
    })();

    window.klipse_settings = {
        selector_eval_scheme: ".language-scheme", 
        klipse_limit: 5,
    };
 
    // Загружаем Klipse только для этой страницы
    (function() {

        function loadScript(src) {
            return new Promise((resolve, reject) => {
                const script = document.createElement('script');
                script.src = src;
                script.onload = resolve;
                script.onerror = reject;
                document.head.appendChild(script);
            });
        }

        // Загружаем всё локально из папки этой страницы
        window.onload = async function() {
            try {
                await loadScript('sicp/js/klipse_plugin.min.js');
                console.log("Klipse готов");
            } catch(e) {
                console.error("Ошибка загрузки Klipse:", e);
            }
        };
    })();
</script>
<link rel="stylesheet" href="sicp/css/codemirror.css">
<style>
    .klipse-result {
        background: #c4f5d3;
        color: #333;
        border-left: 5px solid #72e98b;
        padding: 5px;
        margin-top: 0px;
        margin-bottom: 20px;
    }
    .CodeMirror-line span[cm-text] {
        display: none !important;
    }
</style>

