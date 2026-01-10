### Убрать лишнее окно внутри iframe:

В файлах src/circuitjs/09DBC...cache.js

Найти код:
```
function LM(){ye(this,Fi($doc));(VG(),this.Nb).className='gwt-Frame';this.Nb.src='iframe.html'}
```

Добавить 
```
;this.Nb.style.cssText='width:0;height:0;border:none;display:none;';
```

Вот что должно получиться:
```
function LM(){ye(this,Fi($doc));(VG(),this.Nb).className='gwt-Frame';this.Nb.src='iframe.html';this.Nb.style.cssText='width:0;height:0;border:none;display:none;';}
```

### Пользовательский элемент

[customlogic](https://www.falstad.com/circuit/customlogic.html)
