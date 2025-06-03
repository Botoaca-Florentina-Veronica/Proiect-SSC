# MITM Detector

Acest utilitar este conceput pentru a detecta atacuri de tip Man-in-the-Middle (MITM) pe rețele locale, cu accent pe două tipuri principale de anomalii:

**1. Detectarea ARP Spoofing:**
Scriptul monitorizează tabela ARP a sistemului și identifică situațiile în care aceeași adresă IP este asociată cu mai multe adrese MAC diferite. Acest comportament este specific atacurilor de tip ARP spoofing, unde un atacator încearcă să intercepteze sau să modifice traficul dintre două dispozitive din rețea.

**2. Detectarea IP-urilor Duplicat:**
Scriptul verifică dacă există adrese MAC care apar asociate cu mai multe adrese IP. Acest lucru poate indica probleme de configurare în rețea sau posibile tentative de atac.

**Stocare și monitorizare:**
Toate alertele generate de script sunt salvate într-o bază de date MongoDB, pentru a putea fi analizate ulterior sau pentru a permite integrarea cu alte sisteme de monitorizare și alertare.

**Utilitate:**
- Poate fi folosit ca instrument de securitate pentru administratorii de rețea sau pentru aplicații web care doresc să verifice dacă mediul în care rulează este compromis.
- Poate fi integrat cu sisteme de autentificare pentru a bloca sau semnala login-urile suspecte.
- Oferă o bază pentru dezvoltarea unor sisteme mai complexe de detecție și prevenție a atacurilor de tip MITM.

**Recomandări:**
- Scriptul necesită privilegii administrative pentru a accesa tabela ARP.
- Este recomandat să fie rulat periodic sau la anumite acțiuni critice (ex: autentificare utilizator).
- Pentru o protecție completă, integrați notificări automate (email, SMS) și monitorizați constant alertele generate.

**Scop:**
MITM Detector este un instrument de bază pentru creșterea securității rețelelor locale și a aplicațiilor care depind de integritatea comunicațiilor de rețea.