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

**Либо правка исходников:**

Файл `/circuitjs1/src/com/lushprojects/circuitjs1/client/circuitjs1.java`:

```
public static final boolean shortRelaySupported = false;


```

Файл `/circuitjs1/src/com/lushprojects/circuitjs1/client/CirSim.java`:

```
    public void setiFrameHeight() {
    	/*
		TODO: Disable right additional panel for symbols
		if (iFrame==null)
    		return;
    	int i;
    	int cumheight=0;
    	for (i=0; i < verticalPanel.getWidgetIndex(iFrame); i++) {
    		if (verticalPanel.getWidget(i) !=loadFileInput) {
    			cumheight=cumheight+verticalPanel.getWidget(i).getOffsetHeight();
    			if (verticalPanel.getWidget(i).getStyleName().contains("topSpace"))
    					cumheight+=12;
    		}
    	}
    	int ih=RootLayoutPanel.get().getOffsetHeight()-MENUBARHEIGHT-cumheight;
    	if (ih<0)
    		ih=0;
    	iFrame.setHeight(ih+"px");
		*/
    }
 

```

```
    // TODO: Disable right additional panel for symbols
	//verticalPanel.add(iFrame = new Frame("iframe.html"));
	//iFrame.setWidth(VERTICALPANELWIDTH+"px");
	//iFrame.setHeight("100 px");
	//iFrame.getElement().setAttribute("scrolling", "no");
```

### Добавление своей реализации элемента FullAdderMultiBitElm 

1. Создать файл реализации `/circuitjs1/src/com/lushprojects/circuitjs1/client/FullAdderMultiBitElm.java`



2. Подключить компонет в файле `/circuitjs1/src/com/lushprojects/circuitjs1/client/CirSim.java`:

```
стр. 1030

MenuBar chipMenuBar = new MenuBar(true);
...
chipMenuBar.addItem(getClassCheckItem(LS("Add Full Adder"), "FullAdderElm"));
chipMenuBar.addItem(getClassCheckItem(LS("Add Full Adder Multi-Bit"), "FullAdderMultiBitElm"));

```

```
стр. 5120

    public static CircuitElm createCe(int tint, int x1, int y1, int x2, int y2, int f, StringTokenizer st) {
	switch (tint) {
       ...
       case 196: return new FullAdderElm(x1, y1, x2, y2, f, st);
	   case 198: return new FullAdderMultiBitElm(x1, y1, x2, y2, f, st);       
```

```
стр. 5232

    	if (n=="FullAdderElm")
    		return (CircuitElm) new FullAdderElm(x1, y1);
    	if (n=="FullAdderMultiBitElm")
    		return (CircuitElm) new FullAdderMultiBitElm(x1, y1);            
```

3. Добавить перевод

Файл `/circuitjs1/src/com/lushprojects/circuitjs1/public/locale_ru.txt`

```
"Add Full Adder"="Добавить полный сумматор"
"Add Full Adder Multi-Bit"="Добавить полный сумматор многобитный"
```

### Пользовательский элемент 

[customlogic](https://www.falstad.com/circuit/customlogic.html)
