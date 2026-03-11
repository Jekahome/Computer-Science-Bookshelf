# TEST

<script>
(function() {
    const localMap = {
        // JS скрипты
        'https://cdnjs.cloudflare.com/ajax/libs/codemirror/5.43.0/mode/scheme/scheme.min.js': 'sicp/js/scheme.min.js',
        'https://viebel.github.io/klipse/repo/js/biwascheme-0.6.6-min.js': 'sicp/js/biwascheme-0.6.6-min.js',
        
        // CSS
        'https://storage.googleapis.com/app.klipse.tech/css/codemirror.css': 'sicp/css/codemirror.css',

    };
    
    // Перехватываем fetch
    /*const originalFetch = window.fetch;
    window.fetch = function(url, options) {
        console.log('fetch:', url);
        if (localMap[url]) {
            console.log(`🌐 fetch: ${url} → ${localMap[url]}`);
            return originalFetch.call(this, localMap[url], options);
        }
        return originalFetch.call(this, url, options);
    };*/
    
    // Перехватываем XMLHttpRequest
    const XHR = XMLHttpRequest;
    const originalOpen = XHR.prototype.open;
    
    XHR.prototype.open = function(method, url, async, user, pass) {
        //console.log('XHR:', url);
        if (localMap[url]) {
            console.log(`🌐 XHR: ${url} → ${localMap[url]}`);
            return originalOpen.call(this, method, localMap[url], async, user, pass);
        }
        return originalOpen.call(this, method, url, async, user, pass);
    };
    
    // Перехватываем createElement (для link, script, @font-face)
    /*const originalCreateElement = document.createElement;
    document.createElement = function(tagName) {
        const el = originalCreateElement.call(document, tagName);
        
        if (tagName === 'link') {
            const originalSetAttribute = el.setAttribute;
            el.setAttribute = function(name, value) {
                if (name === 'href' && localMap[value]) {
                    console.log(`🌐 link: ${value} → ${localMap[value]}`);
                    return originalSetAttribute.call(this, name, localMap[value]);
                }
                return originalSetAttribute.call(this, name, value);
            };
        }
        
        if (tagName === 'script') {
            const originalSetAttribute = el.setAttribute;
            el.setAttribute = function(name, value) {
                if (name === 'src' && localMap[value]) {
                    console.log(`🌐 script: ${value} → ${localMap[value]}`);
                    return originalSetAttribute.call(this, name, localMap[value]);
                }
                return originalSetAttribute.call(this, name, value);
            };
        }
        
        return el;
    };*/
    
    console.log('✅ Перехватчик установлен');
})();

    window.klipse_settings = {
        selector_eval_scheme: ".language-scheme", 
         klipse_limit: 5,
    };
</script>

<script>
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



```scheme
;; Пример кода на Scheme
(define (factorial n)
  (if (= n 0)
      1
      (* n (factorial (- n 1)))))
(factorial 5)
```

 

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
</style>
