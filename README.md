# Svar til spørgsmål herunder
### Hvorfor bruger vi Vec<T> til dynamisk allokering?

Vector håndterer automatisk re-sizing, hvor dette ikke er muligt uden en eksplicit re-allokering af array med en givet størrelse i et array. En Vector lever ligeledes på heap modsat et array som lever på stack, hvorved der kan være langt mere og større data på Vectoren end på et array. 

### Hvordan håndterer Rust hukommelsen for denne struktur sammenlignet med en heap-allokeret datastruktur?

Vector er allerede heap-håndteret, så vi ved ikke helt, hvordan vi skal besvare dette spørgsmål.

### Overvej, hvad der sker med elementerne, når stacken popper elementer af.

Elementerne fjernes fra strukturen og størrelsen (men ikke nødvendigvis maks kapacitet) bliver formindsket.

Ligeledes "frigives" de bytes som elementerne fyldte i hukommelsen, men der sker intet andet i de fysiske RAM end at de er markeret som frigivet indtil de faktisk bliver overskrevet. 

