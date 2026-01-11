# CPU Architecture

* [Arithmetic Engine](#arithmetic-engine)
* [Registers](#registers)
* [Instruction Decoder](#instruction-decoder)
* [Calculations](#calculations)
* [Program](#program)
* [Conditions](#conditions)
* [Immediate values](#immediate-values)
* [Turing Complete](#turing-complete)

## Arithmetic Engine

> [!TIP]
> Разблокирует фабрику компонентов, что позволит создавать собственные компоненты для уровней CPU Architecture.
 
Arithmetic Logic Unit (**ALU**) Арифметико-логическое устройство

Коротко что это значит:
* Arithmetic — сложение, вычитание, инкремент, декремент
* Logic — AND, OR, XOR, NOT, сравнения
* Unit — отдельный функциональный блок процессора

> Задача: 
>
> Добавить еще два варианта в схему [Logic Engine](turingcomplete_arithmetic.html#logic-engine):
> * ADD для сложения двух восьмибитных чисел
> * SUB для вычитания двух восьмибитных чисел
>
> Для 8 комбинаций, нам хватит первых трех младших битов
> ```
> V| OPCODE :
> -|---------------
> 0| xxxxx000   OR
> 1| xxxxx001   NAND
> 2| xxxxx010   NOR
> 3| xxxxx011   AND
> 4| xxxxx100   ADD
> 5| xxxxx101   SUB
> ```
>
> [x x x x x D2 D1 D0 ]
>

Реализация ALU через NAND.

![Arithmetic Engine](/Computer-Science-Bookshelf/img/tc/Arithmetic_Engine.png)

Схема созданного вами компонента определяет его функционал, а планировка определяет форму. Имеено по этой причине, нельзя было двигать красные компоненты на схеме, так как они участвуют в форме нового компонента. 

Реализация ALU через OR.

![New_Component_ALU](/Computer-Science-Bookshelf/img/tc/New_Component_ALU.png)
 
#### Circuit Simulation ALU

Необходимые компоненты схемы ALU:
* [3 Bit Decoder](turingcomplete_arithmetic.html#3-bit-decoder)
* [Bit Switch](turingcomplete_memory#bit-switch-tri-state-buffer)
* [Adding Bytes](turingcomplete_arithmetic.html#adding-bytes)
* [8 bit NEG](turingcomplete_arithmetic.html#signed-negator)
* [8 bit NOT](turingcomplete_arithmetic.html#byte-not)
* [8 bit NAND](turingcomplete_arithmetic.html#logic-engine) или OR (но расход mosfet транзисторов больше)


Проверка:
```
Input 1:     00000100 # 4

Input 2:     00000011 # 3
---------------------------
Instructions xxxx0000 # OR
Output       00000111 # 7

Instructions xxxx0001 # NAND
Output       11111111 # 255 или -1 

Instructions xxxx0010 # NOR
Output       11111000 # 248 или -8

Instructions xxxx0011 # AND
Output       00000000 # 0

Instructions xxxx0100 # ADD
Output       00000111 # 7  


Instructions xxxx0101 # SUB
Output       00000001 # 1      
  
```

<div class="sim-wrapper" data-circuit-id="25">
  <button class="sim-fullscreen-btn" data-circuit-id="25">⛶</button>
  <iframe 
      id="25"
      data-circuit-id="25"
      class="sim-iframe"
      src="./../circuitjs/circuit-frame.html?running=0&editable=1&usResistors=0&whiteBackground=true&startCircuit=/turingcomplete/25_arithmetic_engine_alu.txt"
      loading="lazy">
  </iframe>
</div> 

[ALU 8 bit (www.falstad.com/circuit)](https://www.falstad.com/circuit/circuitjs.html?ctz=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-RDGkJibEgADok3QagoShDQQgZU-xaBwpUqKBkaDkRPCoJcwkVDhViBlPKkAOJ053FYSqkANJDiOEgPcqVKjdXsnVVtLVcg9Fmr2eK8A1krWyCFRkNBohOH2o7oSIJ1g0HYDDvag5IoHjdDQI4Kk1hbByR2J0MYobsJmU2KiFCHQ9hxtCAm9ykaZCDHaM4NNtR41UHEtmvVW8FixC6BmqgANS3VVTZW9NGRsJ5NLWg4YjbC3VtpCyWwXUG2jC7c2vuFJbCLwrYOn1rxwRAtCFKWkJiaAgDiXygVqgbWutoOUd4IRnwchUpnARwqJUslyRQUVVrM7Ks3RQVtVLb1MJkJnHVN6jgInvUcAknJM6mtfSWj9NIISZw3b6qghAs33vA-up9LrQMgGkKekIhAhRAZ9a2Cgc0d0YYjkBkN3bpD8lsARvNuxO3LGHfBtsbbjI5FjUO6w2FWajpkExmNBbyMMdoAGUteDy2kcnXUMNt7322H1B2gTRaKARsyECG+E6liCcYx1HjsVxMKck6K5jGGHADoU0HSyMgOoMXAhPJRdxpFTRzXdEzYBlKXrw5hSayI8ayrAfJ-YgCkChjeRBKQKYISx1kGstZABNduKUlRIYMikSsIAc10wM8suceAbM3KddMxLrULI2cwE6l1Dkc1aRSxsZgZnrUJdzLmTCJmmaEl-QVkAoUTP8CdS+hrrlivwa-VK69DX7KdauE6o9YkZC4BTp13ASyD0ZdzMZ7LGwpRaIPflxLmARGdcwJKp9G6GuiPm3F54QH6urdudVhb95L1tdW+os7cWiCXt66tiw+2UARKA2KWz2VLASsvuaWTcXwD7RUomo2iafCfbXD2uc+pqxUHixDl0sVhElLQcjgHrAEe0v9UmJoGxsfo-C9lbAbQGJ+kwIZ7Y8OUqUjhyePgIQo5iCp9laN0Y5z4EcCmeLGYpgLYXBIJnD4uzRtJ5KXncWWkZhpxPICkAx3A0lMTuxc00CTTR7GG50cAZhskNVTJsV-tcCSv9ocUh+zcA2LFsCfRnO8FVDIW3sZWdVdWrZ9WPsVLlFex8QawufdJlZ+tIHhmtLJji8ZLsYe0Bk-NGH2MUe+rS7j8Hntkppd7Sl1uNqpok9C4zEriXfoC-tq7MXpSKkS-55SIzxXKQGNqQL-X00+P6+xnxzXj3hm89Epl0mTXGq4b9+10TPXKK2rS+qRzU6mKx3bXOHj6fY-zvQxUtL2MpgNho8RxsWCGYN9lpdrvrUHww8ug+PPup5-s8wiRxn4-TOPar-KYWYcsPIe5tN+AaKRMP8Hi3OTkMLcTTNSW8bTSHYAzcOHRlImAA63H7CiPoMHOCRAozRiJAriEnFsHAXdE2OCNoXjBArjRabAwvF2UXGiJzIrSaaNN2CeMlGUUbSlBgugtiIjApDMZg0AxzJDApAAWTFzajZzF2oOPhQAgTpjlxCGjS4CZndStUC2CzCwhWlAeWbwiT4GFFigADFEk3A3A1BZAeZRY5YuZ9AJQfYjUDQ7cND4QIBYxxdRdTQHD2CBDl87c5g18uw5dRtZdbBk8ec-CwwBcDtFpFFldsowi4t3FhCGdRJbMPDoinDsI38ki4dJcxdsJHD4imd0ijcxBDdbMkpcAukTRCixBij3DyjvZPtLdzcIxW8CwaiJ4dxsjmjApbMYs4j7cJ48U7c68XxWiC9PRjB6ikwiZfcTQehr8iY78ig2g8lVpK8-A+hh8VJR8-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-3riKdY+zUYAVAj-4SdY+qmSASglAH9Y0uL-ZBO-0o4dZwux8H-lANAGhRwuJseAWgKkJbcLuu3QgX-326YRBCPfcgfXBu5UCduWA1-jgIf4Oo8Bz3ctMwIQGsDp+DAv7jpmwHcCbo4PPgQv1oE8CoecPB8OIOEHw9geW-eAeaRj6UcfuH-VtD8TT6TEwyn-XsNnycQmcIsuggvvp2ig3QS+TiCvujEo6V9xO-WSjnXx3wdZ0BcMWQUeB7aI8IyK-E8LINVLz90edA9Hq2TmC8CeBViRdqPze50C3uvfB1E4OIF8NiglA0IfOW3YOpUCdHXbjPhkCmd682fe-l8Xw6ItxOHxcQQUN8jRoaOpQyrK5AqFMDUBNTSrPZAqE2QqhMgCThULgGCD8hlWaKBUIgFdCGh2QlKNGl06tCNU2uEYd-y4EopJo2nAvEZxRS2BFOLTcwQRyMyvAxAU3LSlpwmEfA9BBHZZpVlx6UhJoVHMob02wiscCh7HDYbXwVAFDpmcTdoZVg+b45FOBQoFvMOGEGDFouQiYSZ1IaUVLiXuevMDQbzZcwkQI1ELMRsApB2e4gS4ksWbwRhyoCI5NHDFuEDgf6yPKhi0zc7zU4u3nDXNQUFpQj2+WQs7jg0JHTMRhpkLAI-yoZ9Dl6JgoHh0KxD0i9UHzZoTg2L40jxOcDYvuxwqENUORcfGpsUOZF6pceQ1GUWlBnbNk5RCNNqjI3RSjsAI47EquqOLbmguyFIyOsuTiIYp-ahoo6lmV7hR8+ECxC7oQBLy+8b6-gXyph38Br1YoYfchBHz0DighuggZpPMjSze58wtvQsADwIB6hHeoYo3jbzl5g91goPYHLGMjG89decOV1oqBTGPRreSY9-Ie1k43B4GIvKMeLzu6RUKknOIATzzV6nd-K1nfygWKzH88c0usLUHFybF1jKx0PHNjkjo7dE2xdPRsdtFtwes9uh4L8HGXwQ+truv9cnuAHzEHgPuHIwnsyHdaxY1GdvaPHDDB7BEcwIeKhvuVexGZ9Q+4x2HuLLLQ9XsQLPcZAGCazYrxvicfMU2DEhlzOtXCMYTzQBIjfu74y+IA3YoTi7uOY0AmgBhGXjr8gE8XHt3HEbxw8QZbsNBLxj3iYJg41MZeJ5YHdwAL8BsehJOrA8mxhYUsq2Om4lZcJRAUsjFQrCkSVcx3MiU93QnbQ8yJgfqpgGCFHAvmwQ30HDHonsTACnOZboIStABNOc+7ZAoJIEkzjpmTXSCYuMgktcmaFEbLtRIwJSFw6mAbLm5QK7gBkq2PDSYG2B5BVO4tYliNWIfjMgiS0XFLF5Ti7mSIeJWEcUVBskwiQutk-zmWz06-FsUIXFyRaLIQLF4IvrdfFzngAUQkofk7pLa1fBHifOUhJNtZ3PFA8wexE-vOhEdhs89JNYliZFNontIo06U-yWWN8pK9+JAyEST3jZwbiPg9OaHOVIUn34yEKuRKdVPuZLiwxNkmCUJ1Sm0oDJe+eMS2GALRgYRvUpyR1Nsm0copEyOjjaJ9a7lowDHKNBNJJTmQPWVzHiYPmI5cTzmOaLiWv16o9sIpLXCKbsVjzFlPxZZHMoZl-xaC0wsUzMnOV8Gll-BuASnLeJn4hkIs943cfeIv4eFimQQpnA+OMDBCwh1k8PK2jbJfjDSoM38bOzH7CNwa9ApdgznAlPFwaQE9GPeOAkITW+1ebwuDKxnbpEhPfJGSiMFqpChxhMkMirhQnCNtBtaMMklMJg-CeGDEn1j6UU4+lSpLpTkd9AKmoTaGmw04lVJgmNRWOjUPqcmQ0HqCPuRo74pLKAgXcpZwZdCStDln-VGZ6U0QTlN5JKTze1ooSRYM5wUy+JPMmvkZOtGCzHeg060TCMT5ulgcc0z4nNNEF2zGic05kN5TdQJjyqLsjlhFXdlOUvZaEL2RqOmnvEeo-smNnNMWnBzfi-VKOSjIPLpNIinshOUAgjmIzSiEcssrHO+nOy-CmcjOfQyjnMhVGhKIuXrELneycqUUoOWHMfi-ErqXsnqE2O6ZJzfpOMdKaTPTkJzWamRNOYnQRkpyFp2c9OYPKaRcT1x2VWyWDIiojzLmQdBGQMBLFgyD2IpSGRFV7BLyc5IeBeevOxkHtTS-kX+nHM9mTRfWC5EPLrL7nh5Vqh83uSPQRlh0E5OxEyiVminPy4JE8hNK-KspNFl588+ajaV3mNEORkcyCR3KEDlTq2fAE-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-tUOx1f3rgVr7EC3oSA-jrH0gH+V5+6vFnlCOQ68dH2iKm7rIiOLwGaR+1QTv722Sndz+uYHEYB3Q6F6DI8BeQGYOmrzViRv1OHp-1tByj4Rkzc7LajnxRZjWGObZJsYRUujfR4bqzWG7KL+jKckY6TvC7PQJj5unfEDVsmrDBjO+BqksY8pdKwyKxvVM1VknK9-6JWMMp7KGNBGxAiNdOWMafArFxjUxzffuWONZ9yV9BvxTMZyM74C56xkZNHzL0Kc1jaCMUT7M2Hk7-juxvSQqyJ1s0uJMMKE2zXgbgmYTtO04ZtChOU6jBzNewazvP5h00TIhhTosYxO4ni0ulJY7jxgYaomkv9LYV4vJM4M1iNyywUIBY4ds4TDexvgXKsrMEY97xmNk2M5NvzmC5VF+LJEgnU0tKewukXsM2UpgMYIpp+fA3gTwN9hGihUymFBqMKBe-ygESgbTDwIrY2fEA+RF1M6mIwbRjI4ToEgYxAZCwulLJAtNPyJwnsafssbn3B8KUmwguYShhiAyqTrKmk-Ip9O6LlaTpjyuRAxjkRMu5CtoUUm9Ns0X4MMOM7GaC4NVvTuxpseCbTOpno+O1I4eHPwHWri0uBA00lDvL9JryppwozNS1NrQa0gIq01ZGlNPzvTz0VcM6e9ObQWzHbb00DRbMeUYz-yhTmzRTNHwCzIVb0ztQU5-FOlEiSLVckayzpYtwytg4lo4O0tplXBng0Qo-JoKlTOdBkT-QbOvLi+DehTuyeL6bQ4TzJ3kTgzxNV7BRx9Ac+0VFEijfjh5-Tm-QnNp6Qg3R2ybqO+XegSAeB6QD4YG3AqkolgGugskazgdbD022bY4bpbza3DjdSgBsG4gEIS63EJhH4acCRH6D4imbskXyN4bAdkRiMHyYnDMFyzCRlOViafBtAboVF-DYkaRNUREuSOIi0duot8qlhgySLlogh0FGx9sdS5Pgh7pCh8EsSjOKjvR2JHwu8urM-cgEvEXodncm4E2ULng1Gd4YLEC7M0vOyxd-kXSyrMBOogF5Bl2Xd8FHli6dFFOvQ-CK0ukjbLIVF2QfELmZD05UaVyy5chNuooaKivS6pa-zRLtLE88yxwqExaXswdMsEZFbVWogGlTlhBo4A7AiHfOjl4ju5aN2OBvOputIdMZ26zGMrHxzninIyGnniBsxsq+HKjRWdndqIveZcWvmcdgl1BSEb-VC7bDLOdI5q6+cpHM79uB7R3ijyEBDW-FCaYa1ydRDKLSFtV7+TNAd2WcKT5Urnge3KlXQ5Qy1hpXAvWtQK1LMcpuRMHgZY1CT0I9kUrwhidXTrQgJXvVFwU5cqFo0QqnCLbJUL8utVUslFfR4xX3reXForQtGgGnnrf12fvxYM1hGzT4+74KtS+u3XIbX+fCMENhuFUUC1KAHitdgWIEdrEuraxCKAQw3AR5EQFXRcs4E24RKVkxaiHtNk3VqCulK4KaJk96AeRI2BSiNqtNzmbhVZlaSK+uZX39NwHK0zLysfzsrQNHmxsbdSOWSRb5RI+NbbkQi1cHFm7dAdIOmX94vnR6yrb87QyrrFSkrFz1yXxXZ5+N8g+iOyow22lXVjA0zebNpWC5PN9s2lY7Zi24DxApyHQfiUqF9keXV9gufi3sHJlq5lLdweOXAjJR31-ZVdYevnLkeFJ9WzcuR5AIZrgIt5eTd3OU3XlhIlvRVc92EjzzjtsuW8q2pp2BrIdr-Jbt8vMRvlDB3qHAd6UhR+4wKq9nCGzq5IfR+ASbRsDsMzaHDGgJw8ioW3uGPaal9ikxHxWBBP62F0i3lwRtqXF0Slzi0xaZV7WRkmNpKIxZIvMXLONN6m+DrBvpHCjiRmqwEb1vmR170O2SwVbIhc3h9it0ffhqqPIAr2G1o1ap2R3SWijEtxYwmlmpn2IjgV3qoXKthaW5EgDuPS7KGSFyJwwDjSOA--1S7DLbqAif-dSmFzCpWlh2ag5stS67LTsjyxtJwfBK3UVSFRUA-0s3AQHhNqB2Q-QkDEFdoyf+7hOitIP+rhUvm2QnyVk6F5CaPSXdYwdcSo5ns1aazVWkJW+JwjznGYpLGyS15Sk85b5ycl1Xhpm82qeiaUnTXgcassk51LipSFNH7VjWZderFaOeHC1shCnKl3J3HeGEt+bJM9kJprH38+WcXfDo2OGpzjg+VVM2XlTMqu19h+0TgU+PEFhKemyVgaTYLFGQCpXno9L2h7dHPUI6wY8vne0qFzD-Uck+LvugMD+E4qAD1ScsLlkYYWhQRMBtzlCnd080L-Y6NSLMpmtxJ50mWTFzmJ+1xAqw-oVoOMbjgHxyXPaeFV5Jiimycosof4Q+nyCuESg6+smOnr-YkZGM50VNyFHEz6xSiJcfzPrFzKjBxM62vrOab6zhK9w7xsA9Np1DqORFREIl4ZbnT4Z7o7nt73BL+GmA4rLwO+c8nX1zR290Fu62-HGSr558+kfa3bHtVQ56Uvkc6LFnTz2R2XM2cu3vJRtw500qUl65Olg2SajXWfCHJoL2iVgwlqS2B31zpdumSk+Mtki9HzzmOW8taekKenaIv55S+mtxcUHlLnReS9OUQuJDG02O0pPjsbTE78j5O2jzPAuyt+XouUMxUaCqNXgvy57JPRIA+j-BTofFcIlIRTb7Dc2lwwPeQsfcYUo9xzvuknsGni9jTw5AdtvtQHF744ZlV08QKEpKnJ2zezU7SfpSbXd27ixtLbnLOFb89pW2a4ONkIdnfCNqE66ngP3xEScQ1cslaMaSUdDR8x3whovLTfttz5S3-cVxWuNIby1J0NMTv0vzOBeQWyK+mcN4xnB8+l4QpsDuvsRrr34es4GDkuYJeIhR8y8dho4TnZI3ByVLWm0oAlGdp5WnkulPTHl4ebXAAQeWYowl2uZt22-WkwuckZzgQyHhSZ9uPxLTKeTTgkdNvZp1+etzCPqkgjq3QPeqSrjiYAuDmfCXd79JhHDvxc+bq+Y7CPfKS4mRyqXH25rc3imXN4lXJe6Nnt5Ux+OdmRrlmnm9f3VLqd3m5sDZc6nYHuJ7sxJyBT48fb4txYrZfRyFQABBhSm7ZXbRQRGbxKcRKw-GXqOLuRKQTJmH6dfIzb3m9RzI+nCfu-eVjisPY70fek39z4cyvJs2Bvjk0pYfZFuERn88rN7XE8NeFk2Fibwz4RCOvxfDfhKBEESsXM5o5tOQ+KhqJ77yXDPhQXdYdHw7bBE+D-pbKtp7AaCGC2HhberNj8raeSFkhmNtp+D2zYnKwRJQ4oeqL2e1Dqhoz+UQQWKHI9yjA-Ne6flGMvzaEDkbqONE5p1Lby80YxRxCGpTISoQ44BglwgBnkkX2dKfRQbkAkdofJL8uVyzgc5Q9RL9LFFBTgpBqokLCCDU4rNl4v0KpFI3RSwKvh6jVcNH2gS9ujavPAc6OwiVAQqA3CX05OckWFkQBiCYjRUo2LlDe7pjNBMQA7eVCvSiqCkB8EXeUSM3TlRViHp+CH+fqXm39cgG0615qdvD2kGrlVx4oV+yuVZqlBU7K5VUqAFSdpW0uavk9vIOvcvhQa25V6GO5YOtLUbO+cllhvND6dVYXrgrHRL954w4up0zNqIPvA0NZ0XvOorEPjW4D9VtxRYoO51cFuYuro+0f51G0oTTKULgLqfe5arAdirE+M0amxwkhqp9FZSfkJQ78myPY0R3qvQfb8gWB1lkV1qTavpdMZplKzw3nTwjNVXxwIA97RQXwL7WKNmDzTUIYaOal9mxOOZi+H1UqV9lzvOWtjX3U-+dvONHMcrXz1EF+wf9qGy-5fvPF9FLN4CikKilcjkpW-3JvrWTqHt-AesRXz9sJzwaXw+mk9v9GzNRXkDBCoj7ssIPMD+rTlF7zlx4VGPffPjF8Io+Y0S55x+ue2jg20Y5T+TPY-q856i4s6ghkffYj9X-I9Wr1Qejm1Zpc76lWAjMgdysEQI8d-Tvvq7L2edUumhhKn5w1rt2WBEK1neXzvnv+0Wj9R-iO81-3yTed8j-q-UuwP2Lp++W27a3wDv4X7eoOoC57zgW6X45sDRm-2-oW+X+IGbRMg5Ih6gP+7+u2W-aVoGsNZLsPVPfuUVrjGxbPPQUrzp6P7VcKgBmOzdUPDh2xT+p2uevHtWZ3CC-sAHlUm-s-6YC4vg+Yr+TfIn6eWh-ksK7GLZg1SZAAJkTZ9WD0G+aP+H5mf5Sij-uIYe+DOH5Qv+z-uvJ+U2BlmALSceh-6H+VATGwABV-lrpF+DOKoxJ+AtI7qB09UCQZ5+ZaM74W6F-hQYtEmQJQaoBKAWOhByNBgtA9ENvqcK6Uw1kMSJ+ZAflJM6YAXQE5IUBPQFMBf9OiQPU-ASCwu6cAaNgtErntlTyBCChQbE8CeMoH3kNHhoHqBvDHDCAMLYNOB3MBpCbrR6w6l8Tc6PauyYWB+5lBpn8n2hUhBB-kkEGHQIQRiQOB9gUWwkoi1EUgaagUK5r2QLgdMwpBbINjQpCo6n6AWUHzDkFkg3mkfyBaAQD4FWe2NG9BlBi7K1rXWAQQewNMpkFKxdaRgDzSlyrPq9iIuKwMnSjyYcDaJcoVACsj8oq6GDR9a4kOJCqa4QeprC0RQdlQSaydMRQdsOrEpqfUHlAkHkMh6pz5rGEQaUQ6sJQkz5DBeanNBIaowb2CnMOxH+xGBlajGpzBLfHayeg5TP7iea-xFEEuUkJkEEuUWFAz4uUZlAcE36VaqVD8EJwfpx1BW8BpQKUO-BME+eEBHFjiE-wQUEQhcAiCEwhVtFxo3wCIWCHqcX2tkCohbwYCEhBUIdyCIhR7H+QlCWIb8E+eVuHiHcCoIkspRwWZCsriC9eKfI0hiHlEBcC9-AyGpin7pwL1CzeIvrSqTIe-Ish3IWEApiADH279oAwjyEpiC7vO4oCIAsKGDuc4Je5TCQoeyHU4fbkAKCCA3uHgpQqob0xjusTLyFyePciJ68hH4rcLGK8oQn58eIZDJ4piMEt+6HuvIV+4wegHryFCS7-Hyz800LMqFyhlUB4zAslDFyFyhB6tFBdq8wKyGRgZ2jfig6KaBKFpiVtFO5Hq4YSqGHMrkHsx1CcoY4TrMorAQJxhKzMsyFMAglwLCIEcADAX4ZZBtQaohFECGIU2UCCTU4qqIqEPeUwXhTBBO3sEGC8eUGeRjqsVJ6BWiWfBeyCQdwbewU+twSyA1o76IkE-aRDPjiQkPtNkIIg9cmEGYYfRh1SNha4ZOAM+bTKhLbhj-LuGTg3wTZJeMd+NCzDcbTHtBAAA)


---

## Registers

Пришло время создать свой главный проект, реализующий компьютерную архитектуру OVERTURE. Это будет настоящая машина, полная по Тьюрингу, истинный компьютер во всех смыслах!

Этот уровень — реализация инструкции MOV между регистрами, где адреса источника и назначения закодированы в байте инструкции.

> Задача: Создайте схему распределения/получения данных соответствующую спецификации 
>
> На схеме вам дается:
> * 8-ми битный компонент для получения инструкции 
> * 5 ячеек памяти в виде 8-ми битных регистров
> * внешний вход 8-ми битных данных
> * внешний выход 8-ми битных данных
>
> На этом уровне вам нужно создать схему которая может копировать из источника в место назначения.
> 
> Байт инструкции на этом уровне определяет источник Source и место назначения Destination.
> 
> Инструкция — это 8 бит, но используются 6 из них:
>
> ```
> [ x  x | S2  S1  S0 | D2  D1  D0  ]
> [ x  x | Source     | Destination ]
> ```
>

![REG](/Computer-Science-Bookshelf/img/tc/REG.png)

> В качестве источника и получателя может выступать один из 6 регистров, которые мы обозначим соответственно REG 0,... REG 5.
>
> Кроме того, эта карта имеет отдельный входной компонент, который может быть источником, и выходной компонент, который может быть пунктом назначения.

Назначение адреса для источника Source:
```
OPCODE Source:

S2 S1 S0
--------------
0  0  0  REG 0
0  0  1  REG 1
0  1  0  REG 2
0  1  1  REG 3
1  0  0  REG 4
1  0  1  REG 5
1  1  0  INPUT # использовать внешний вход
1  1  1  UNUSED
```

Назначение адреса для назначения Destination:
```
OPCODE Destination:

D2 D1 D0 
--------------
0  0  0  REG 0
0  0  1  REG 1
0  1  0  REG 2
0  1  1  REG 3
1  0  0  REG 4
1  0  1  REG 5
1  1  0  OUTPUT # использовать внешний выход
1  1  1  UNUSED
```

Например такие инструкции: 
* `00000110 => Source 000 (REG 0) и Destination 110 (OUTPUT)` что означает взять данные из `REG 0` и переслать их во внешний `OUTPUT`
* `00011001 => Source 011 (REG 3) и Destination 001 (REG 1)` что означает взять данные из `REG 3` и переслать их в `REG 1`
* `00110110 => Source 110 (INPUT) и Destination 110 (OUTPUT)` что означает взять данные из внешнего `INPUT` и переслать их во внешний `OUTPUT`

![Registers](/Computer-Science-Bookshelf/img/tc/Registers.png)

---

## Instruction Decoder

Декодер 2 на 4

Схема, которую вы построили на уровне Registers, может копировать значения между регистрами, в то время как "Арифметический Блок" (ALU) может выполнять различные операции над 2-мя входами. 

Но вам нужно делать и то и другое в одной и той же схеме. 

> Задача: Постройте "декодер", который будет определять в каком режиме наш компьютер находится, основываясь на 2-х битах которые вы до сих пор не использовали.

Что бы отличать 4 инструкции MODE, они будут кодировать первыми (старшими) двумя битами:

```
[ M1 M0 | S2 S1 S0 | D2  D1  D0  ]
[ MODE  | Source   | Destination ]

OPCODE MODE:
---------------------------------
00xxxxxx Immediate values
01xxxxxx вычислить (ALU) CALC
10xxxxxx копировать COPY
11xxxxxx условия Conditions
```

Определите текущий режим работы по входным данным, затем отправьте 1 на соответствующий выход.

![Instruction Decoder](/Computer-Science-Bookshelf/img/tc/Decoder_2_to_4.png)

Или используйте Byte Splitter для получения доступа к исходным битам, а затем 3-битный декодер для декодирования высоких бит на четыре выхода.

![Instruction Decoder](/Computer-Science-Bookshelf/img/tc/Turing_Complete_Instruction_Decoder_2_to_4.png)

---

## Calculations

Пришло время объединить "Арифметический Блок" (ALU) который вы сделали ранее [Arithmetic Engine](#arithmetic-engine) и схему регистров [Registers](#registers). 

Вычислительная схема была сохранена (ALU) и декодер 2 на 4 (DEC) в заводе компонентов и теперь могут быть добавлены как компоненты в схему. 

Если вы забыли какое соединение что делает, посмотрите на схему в заводе компонентов.
 
> Задача: Используйте декодер (Декодер 2 на 4) который вы построили ранее для OPCODE для MODE, чтобы понять что делать с регистрами `REG1,REG2,REG3`: копировать COPY (opcode = 10) или вычислять (ALU) CALC (opcode = 01). 
> (для упрощения уровня, в режиме CALC регистры заданы жестко т.е. конкретные, их пока не адресуем) 
>
> Вот 4 режима (напоминание):
> ```
> [ M1 M0 | S2 S1 S0 | D2  D1  D0  ]
> [ MODE  | Source   | Destination ]
>
> OPCODE MODE:
> ---------------------------------------
> 0 0 Immediate values (не нужен сейчас)
> 0 1 вычислить (ALU) CALC
> 1 0 копировать COPY
> 1 1 условия Conditions (не нужен сейчас)
> ```
> 
> На этом уровне вам нужно беспокоиться только о режимах копирования COPY и вычисления (ALU) CALC
>  
> Когда вы находитесь в режиме вычислений (ALU) CALC, используйте `REG 1` и `REG 2` в качестве входов, а результат сохраните в `REG 3`.
> 
> Не забудьте что у декодера 3 на 8 есть выключающий бит при HIGH сигнале, так как нам в режиме CALC не нужны из OPCODE инструкции Source и Destination, ведь мы жестко фиксировали работу с регистрами `REG1,REG2,REG3`.
> Но в режиме COPY, декодеры 3 на 8 должны использоваться для адресации регистров либо внешнего входа/выхода.
>
> Еще, в этом уровне, модифицированные регистры, у них есть дополнительно ножка "Always output", т.е. всегда можно его прочитать без необходимости выставлять сигнал HIGH на ножке Load.

Для двух старших бит можно использовать декодер 2 на 4 который мы построили ранее на уровне [Instruction Decoder](#instruction-decoder) он тоже принимает 8 бит но реагирует только на первых два старших, в принципе его можно переделать, для этого нам и дали завод компонентов, заменим 8-ми битный вход на 2-х битный. Или можно взять еще один декодер 3 на 8, но это избыточный вариант. 

Наш компонет ALU принимает 8 бит инструкцию, входом для нее будет служить младшие биты которые мы используем для адресации выхода Destination: `D2  D1  D0`

(COPY тут не реализовано)
![Calculations](/Computer-Science-Bookshelf/img/tc/Calculations1.png)

Либо можно вообще не заниматься двойным передокированием младших битов инструкции `D2  D1  D0`, и напрямую пустить в ALU все 8 бит инструкции. 
И все же нужно верно использовать инструкцию COPY, которая должна управлять активацией Source, Destination

![Calculations](/Computer-Science-Bookshelf/img/tc/Calculations2.png)

А вот переделанный декодер для входа на 2 бита вместо 8 бит

![2bit_input_decoder_2_to_4](/Computer-Science-Bookshelf/img/tc/2bit_input_decoder_2_to_4.png)

---

## Program

Компонент ввода инструкций был удален. Он заменен компонентом программирования Program Memory. 

Раньше мы использовали ручной ввод инструкции (Instruction Input) прямой передачей 8 бит. 
Теперь нам выдают Program Memory (ПЗУ **ROM** (Read-Only Memory)) который содержит в своей постоянной памяти 8-ми битные инструкции.
Что бы их достать нам нужно их поочередно брать по индексу, для этого нам нужно использовать счетчик который будет с каждым такстом увеличиваться и мы будем получать следующую инструкцию из блока Program Memory.

В блоке Program Memory инструкции хранятся в определенной последовательности.
Это и есть последовательное выполнение программы.

Для этого уровня вам необходимо использовать компонент 8-ми битный счётчика, который вы разблокировали ранее.

![Program](/Computer-Science-Bookshelf/img/tc/Program.png)

---

## Conditions


> Задача: 
> На этом уровне на вход подаётся значение и 3 бита условия (8 возможных комбинации).
> 
> 3 бита определяют условие, как показано ниже. 
> 
> Проверьте значение по выбранному условию и выведите 1 если оно выполняется, иначе 0.
> 
> ```
> V| OPCODE:    Выведите 1 когда: 
> -|-----------------------------------------
> 0| 0  0  0    Никогда т.е. ничего не делать
> 1| 0  0  1    Если значение = 0
> 2| 0  1  0    Если значение < 0
> 3| 0  1  1    Если значение ≤ 0
> 4| 1  0  0    Всегда, просто вывести 1 
> 5| 1  0  1    Если значение ≠ 0
> 6| 1  1  0    Если значение ≥ 0
> 7| 1  1  1    Если значение > 0 
> ```
 
Убедитесь, что вы находитесь в режиме -1 , а не в режиме +255.

**Связь с условными переходами (JMP / JZ / JN)**


Есть:
* VALUE — обычно результат ALU (последняя операция)
* OPCODE (3 бита) — код условия
* Выход Conditions — COND_OK 
    * COND_OK = 1 → условие выполнено → переход разрешён
    * COND_OK = 0 → переход запрещён

> [!SUCCESS]
> Нужны всего 2 флага:
> * ZERO (**Z**) это VALUE=0 получить можно так: все биты через OR потом NOT т.е. NOR
> * NEGATIVE (**N**) это просто старший бит VALUE (бит 7)
> 
> И далее нужно получить POSITIVE (**P**): `P = NOT N AND NOT Z`


Схема Conditions — это и есть “блок условий процессора”. Он отвечает на один вопрос: разрешён ли переход? (1 или 0)

| Ассемблер | Условие | OPCODE      | Проверка |
| --------- | ------- | ----------- | -------- |
|           | никогда | 000         | 0        |
| JMP       | всегда  | 100         | 1        |
| JZ        | == 0    | 001         | Z        |
| JNZ       | != 0    | 101         | !Z       |
| JN        | < 0     | 010         | N        |
| JP        | > 0     | 111         | P        |
| JLE       | ≤ 0     | 011         | N OR Z   |
| JGE       | ≥ 0     | 110         | !N       |

```
O0 = C0 & 0
O1 = C1 & Z
O2 = C2 & N
O3 = C3 & (N | Z)
O4 = C4 & 1
O5 = C5 & !Z
O6 = C6 & !N
O7 = C7 & P

COND_OK = O0 | O1 | O2 | O3 | O4 | O5 | O6 | O7
```

![Conditions](/Computer-Science-Bookshelf/img/tc/Conditions.png)

---

## Immediate values

Нам нужен способ напрямую передавать числа из нашей программы в регистры. 

Ключевая идея уровня - Инструкция сама является данными!
(Как в уровне Calculations, мы использовали младшие биты предназначенные для адреса Destination `xxxxxD2D1D0` в режиме MODE `01` не для адреса Destination, а для блока ALU)

> ```
> [ M1 M0 | S2 S1 S0 | D2  D1  D0  ]
> [ MODE  | Source   | Destination ]
>
> OPCODE MODE:
> ------------
> 0 0 Immediate values непосредственные значения
> ```

> Задача: При режиме "Immediate values" преобразовать 6 младших бит инструкции в байт и передать его в `REG0`
>
>
> Если линии остаются активными, когда не должны — будет короткое замыкание. 
> Это значит: При Immediate Mode: только Immediate должен писать в REG0. 
> В других режимах: Immediate вообще не должен влиять на шину
>
> `IMMEDIATE = NOT M1 AND NOT M0`

Если 6 бит инструкции после бит MODE и есть наши данные `S2 S1 S0 | D2  D1  D0` то мы можем иметь значение от 0 до 63 включительно (`xxxxxx = 2⁵ + 2⁴ + 2³ + 2² + 2¹ + 2⁰ = 32 + 16 + 8 + 4 + 2 + 1 = 64`)

Тогда мы берем 6 младших бит инструкции и превращаем их в байт, и записываем в `REG0`

Нам нужно:
* используя компонент `Switch 8 bit` (MUX) решить какие данные пустить в `REG0`
* используя компонент переключателя 1 bit (подойдет и `Switch 8 bit` (MUX)) разрешить регистру `REG0` сохранить данные, так как биты адресации Destination мы теперь используем как часть данных
 
![Immediate values](/Computer-Science-Bookshelf/img/tc/Immediate_values.png)
 
---

## Turing Complete

> [!TIP]
> Разблокирует компонент RAM (ПЗУ).
 

В инструкции два старших бита M1 M0 отвечают за режимы MODE, в котором нам нужно реализовать `11xxxxxx` Conditions. 

До этого момента все программы ограничивались выполнением байт за байтом.

До сих пор только код из Program влиял на память (REG0,...), теперь память должна влиять на код. 

До этого:
* PC (Program Counter счётчик) всегда делал PC = PC + 1
* Код → влиял на данные
 
С добавлением условной логики Conditions, наш компьютер теперь может выполнить любой алгоритм и вычислить все что вычислимо.

Теперь:
* Данные → могут влиять на код
* Мы можем изменить PC из инструкции
* Это и есть условные переходы (branch / jump)


Последний недостающий кусок CPU которую нам нужно добавить, это механизм для изменения текущего значения счётчика через **инструкции**, при определённых услових.

В режим Conditions мы попадаем при инструкции `11xxxxxx`

В этом режиме, значение `REG3` проверяется на условие заданное тремя младшими битами инструкции `xxxxxD2D1D0`.  
Если условие выполняется, мы записываем значение `REG0` в счётчик.
Изменение значения счетка с помощью условия означает, что мы можем пропускать инструкции основываясь на условиях или запускать инструкции в цикле.

Эти условия соответствуют компоненту Conditions который был сохранён в завод компонентов:
```
0 0 0 Никогда т.е. не должны ничего делать
0 0 1 Если REG3 = 0
0 1 0 Если REG3 < 0
0 1 1 Если REG3 ≤ 0 
1 0 0 Всегда
1 0 1 Если REG3 ≠ 0 
1 1 0 Если REG3 ≥ 0
1 1 1 Если REG3 > 0
```

Реализация:
* Данные для проверки условий в компоненте Conditions. Берём `REG3` — это результат вычислений (для этого нужен 1 битный переключатель (подойдет и `Switch 8 bit` (MUX)) чтобы в компонент вычисления условий Conditions пошли данные из `REG3`)
* Если условие = 1 → берем данные с `REG0` и записываем в PC
* Иначе → PC просто увеличится на 1, как обычно


![Turing Complete](/Computer-Science-Bookshelf/img/tc/Turing_Complete.png)


<details>

<summary>компонент RAM в circuitjs</summary>

[RAM](https://www.falstad.com/circuit/circuitjs.html?ctz=CQAgjCAMB0l3BWcMBMcUHYMGZIA4UA2ATmIxAUgoqoQFMBaMMAKABYxsQM82QG22FCDxhC-QcKp4R4EHzBUm-FCwCyIvEpRsqxMMIY6qVFNATrN2-CH2G0M0+ct4t-MBnF33nqCDMWADJWEkK2BqFScgBmAIYANgDOdNSQLm5GGMLemVEB6Uq4fDlFfvkaroXYXhEM2NVlzhUZ1TIlhI7+TSECcOGGuiZdQdy8KrrcDUYTQ3FJKUhpwTx8RjY44mudEHPJqSzLY0y+Gz7iVDsJe4sHo6uSk5sPFyC7C1C3KypZj99RVG99odVqVTnVIHxZld3ks7vx6uIwQi-ADoUC4XUOr9MZ1UfN0V9elQwYMUa80TcOMRwJwnmFmFiBGFpLdmFMHDSISobC9ATdgmzNsZOathbyKR8BbS-iKZeL8ZSUBAwGwdPwEBEVRMGBqop1Fn4DUh+eBVaKEOJOMR7BayXyPlTwAg0BI3GAEM7XUM8CwAEqmtU6zXYa3qzX+ZBOJABFHOAXO7Rq92eoxq+XXSUB0VJkP2NMxCWwlWBlCucC5lRlqEKzPFkEuq2GIRDPEZotm+Gexud-7kmvtwPYMbdupjatt1kdgRicuh6fnPz2gf3NgyEdsVd2wuTwOri4VgRuccwnerMChkfnqKXfushPuC8ewxXvw3icADxpGCQAgwypQCBcAIWJ8GqAAiACWiQAA7xLEACeLCflo1K-nweCEAMf7yP4fC+gAgmoSGmhw6qAU6uBkVQoF8AAwgA9gArgAdgALnQABOLBAA)

RAM:
A0, A1, A2 — это адресные входы (куда подаётся адрес ячейки)
D0, D1, D2... D7 — это выходы данных (откуда читается содержимое ячейки)

 

**Правильная логика управления RAM:**

| Сигнал | Значение | Режим работы |
|--------|----------|--------------|
| **WE = 1** | HIGH | **ЗАПИСЬ** — данные с входов D0-D7 записываются в память |
| **WE = 0** | LOW | **ЧТЕНИЕ** — данные из памяти выводятся на выходы D0-D7 |
| **OE = 1** | HIGH | **Разрешение выхода** — данные появляются на выходах |
| **OE = 0** | LOW | **Высокий импеданс** — выходы отключены (Z-состояние) |



RAM:
```
0: 0 0 0 0
7: 15
```

Вход это адрес ячейки:
```
A2=1 # 2²=4
A1=1 # 2¹=2
A0=1 # 2⁰=1 
 
Это означает адрес: 111 в двоичном = 7 в десятичном.

2² + 2¹ + 2⁰ = 4 + 2 + 1 = 7
```


Выход значения формируется из 8-ми бит:
```
D7=0
D6=0
D5=0
D4=0
D3=1 # 2³=8
D2=1 # 2²=4
D1=1 # 2¹=2
D0=1 # 2⁰=1
```

2³ + 2² + 2¹ + 2⁰ = 8 + 4 + 2 + 1 = 15

Для загрузки файла, RAM ожидает бинарный формат:
```
printf '\x00\x00\x00\x00\x00\x00\x00\x0F' > data.bin

Что создаст файл с:
0: 0 0 0 0 0 0 0 15

Что тоже самое:
7: 15

адрес: значение
```

Формат данных:
```
1: 4
2: 10

трансформируется в:
1: 4 10
```


</details>

 
---

<!-- Feedback -->
<!-- Read the Formbutton docs at formspree.io/formbutton/docs. See more examples at codepen.io/formspree -->
<!-- <script src="https://formspree.io/js/formbutton-v1.min.js" defer></script> -->
<script>
  window.formbutton = window.formbutton || function() {
    (formbutton.q = formbutton.q || []).push(arguments)
  };
  formbutton("create", {
    action: "https://formspree.io/f/xkogdkjd",
    title: "Feedback",
    fields: [
      { 
        type: "text", 
        label: "Name:", 
        name: "name",
        required: true,
        placeholder: "Your name"
      },
      {
        type: "textarea",
        label: "Message:",
        name: "message",
        required: true,
        placeholder: "Please share your thoughts...",
        rows: 5
      },
      {
        type: "file",
        label: "Attach file (optional, max 10MB):",
        name: "file",
        required: false,
        multiple: false,
        accept: "image/*,.pdf,.doc,.docx,.txt"
      },
     { 
        type: "email", 
        label: "Email (optional, for reply):", 
        name: "email",
        required: false,
        placeholder: "your@email.com"
      },
      { type: "submit" }      
    ],
    styles: {
      title: {
        backgroundColor: "#333",
        color: "#fff"
      },
      input: {
        borderBottom: "1px solid #CCC",
        borderRight: "1px solid #CCC",
        padding: "5px 0"
      },
      button: {
        backgroundColor: "#4a5568",
        color: "#fff"
      },
      form: {
        backgroundColor: "#f7fafc",
        maxWidth: "400px"
      },
      submitInput: {padding: "0.75em 1em"}
    },
  });
</script>

<style>
table {
  margin: 0px !important;  
  border-collapse: collapse;
}
</style> 