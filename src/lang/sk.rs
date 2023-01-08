lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Stav"),
        ("Your Desktop", "Vaša plocha"),
        ("desk_tip", "K svojej ploche sa môžete pripojiť pomocou zobrazeného ID a hesla."),
        ("Password", "Heslo"),
        ("Ready", "Pripravené"),
        ("Established", "Nadviazané"),
        ("connecting_status", "Pripájam sa na RusDesk server..."),
        ("Enable Service", "Povoliť službu"),
        ("Start Service", "Spustiť službu"),
        ("Service is running", "Služba je aktívna"),
        ("Service is not running", "Služba je vypnutá"),
        ("not_ready_status", "Nepripravené. Skontrolujte svoje sieťové pripojenie."),
        ("Control Remote Desktop", "Ovládať vzdialenú plochu"),
        ("Transfer File", "Prenos súborov"),
        ("Connect", "Pripojiť"),
        ("Recent Sessions", "Nedávne pripojenie"),
        ("Address Book", "Adresár kontaktov"),
        ("Confirmation", "Potvrdenie"),
        ("TCP Tunneling", "TCP tunelovanie"),
        ("Remove", "Odstrániť"),
        ("Refresh random password", "Aktualizovať náhodné heslo"),
        ("Set your own password", "Nastavte si svoje vlastné heslo"),
        ("Enable Keyboard/Mouse", "Povoliť klávesnicu/myš"),
        ("Enable Clipboard", "Povoliť schránku"),
        ("Enable File Transfer", "Povoliť prenos súborov"),
        ("Enable TCP Tunneling", "Povoliť TCP tunelovanie"),
        ("IP Whitelisting", "Zoznam povolených IP adries"),
        ("ID/Relay Server", "ID/Prepojovací server"),
        ("Import Server Config", "Importovať konfiguráciu servera"),
        ("Export Server Config", ""),
        ("Import server configuration successfully", "Konfigurácia servera bola úspešne importovaná"),
        ("Export server configuration successfully", ""),
        ("Invalid server configuration", "Neplatná konfigurácia servera"),
        ("Clipboard is empty", "Schránka je prázdna"),
        ("Stop service", "Zastaviť službu"),
        ("Change ID", "Zmeniť ID"),
        ("Website", "Webová stránka"),
        ("About", "O RustDesk"),
        ("Slogan_tip", ""),
        ("Privacy Statement", ""),
        ("Mute", "Stíšiť"),
        ("Audio Input", "Zvukový vstup"),
        ("Enhancements", ""),
        ("Hardware Codec", ""),
        ("Adaptive Bitrate", ""),
        ("ID Server", "ID server"),
        ("Relay Server", "Prepojovací server"),
        ("API Server", "API server"),
        ("invalid_http", "Musí začínať http:// alebo https://"),
        ("Invalid IP", "Neplatná IP adresa"),
        ("id_change_tip", "Povolené sú len znaky a-z, A-Z, 0-9 a _ (podčiarkovník). Prvý znak musí byť a-z, A-Z. Dĺžka musí byť medzi 6 a 16 znakmi."),
        ("Invalid format", "Neplatný formát"),
        ("server_not_support", "Zatiaľ serverom nepodporované"),
        ("Not available", "Nie je k dispozícii"),
        ("Too frequent", "Príliš často"),
        ("Cancel", "Zrušiť"),
        ("Skip", "Preskočiť"),
        ("Close", "Zatvoriť"),
        ("Retry", "Zopakovať"),
        ("OK", "OK"),
        ("Password Required", "Vyžaduje sa heslo"),
        ("Please enter your password", "Zadajte vaše heslo"),
        ("Remember password", "Zapamätať heslo"),
        ("Wrong Password", "Chybné heslo"),
        ("Do you want to enter again?", "Chcete ho znova zadať?"),
        ("Connection Error", "Chyba spojenia"),
        ("Error", "Chyba"),
        ("Reset by the peer", "Odmietnuté druhou stranou spojenia"),
        ("Connecting...", "Pripájanie sa..."),
        ("Connection in progress. Please wait.", "Pokúšam sa pripojiť. Počkajte chvíľu."),
        ("Please try 1 minute later", "Skúte znova za minútu, alebo ešte neskôr"),
        ("Login Error", "Chyba prihlásenia"),
        ("Successful", "Úspech"),
        ("Connected, waiting for image...", "Pripojené, čakám na obraz..."),
        ("Name", "Názov"),
        ("Type", "Typ"),
        ("Modified", "Zmenené"),
        ("Size", "Veľkosť"),
        ("Show Hidden Files", "Zobrazovať skryté súbory"),
        ("Receive", "Prijať"),
        ("Send", "Odoslať"),
        ("Refresh File", "Aktualizovať súbor"),
        ("Local", "Miestne"),
        ("Remote", "Vzdialené"),
        ("Remote Computer", "Vzdialený počítač"),
        ("Local Computer", "Miestny počítač"),
        ("Confirm Delete", "Potvrdenie zmazania"),
        ("Delete", "Zmazať"),
        ("Properties", "Vlastnosti"),
        ("Multi Select", "Viacnásobný výber"),
        ("Select All", ""),
        ("Unselect All", ""),
        ("Empty Directory", "Prázdny adresár"),
        ("Not an empty directory", "Nie prázdny adresár"),
        ("Are you sure you want to delete this file?", "Ste si istý, že chcete zmazať tento súbor?"),
        ("Are you sure you want to delete this empty directory?", "Ste si istý, že chcete zmazať tento adresár?"),
        ("Are you sure you want to delete the file of this directory?", "Ste si istý, že chcete zmazať tento súbor alebo adresár?"),
        ("Do this for all conflicts", "Všetky konflikty riešiť týmto spôsobom"),
        ("This is irreversible!", "Toto je nezvratná operácia"),
        ("Deleting", "Mazanie"),
        ("files", "súbory"),
        ("Waiting", "Čaká sa"),
        ("Finished", "Ukončené"),
        ("Speed", "Rýchlosť"),
        ("Custom Image Quality", "Vlastná kvalita obrazu"),
        ("Privacy mode", "Režim súkromia"),
        ("Block user input", "Blokovať vstupné zariadenia užívateľa"),
        ("Unblock user input", "Odblokovať vstupné zariadenia užívateľa"),
        ("Adjust Window", "Prispôsobiť okno"),
        ("Original", "Pôvodný"),
        ("Shrink", "Zmenšené"),
        ("Stretch", "Roztiahnuté"),
        ("Scrollbar", "Posuvník"),
        ("ScrollAuto", "Rolovať Auto"),
        ("Good image quality", "Dobrá kvalita obrazu"),
        ("Balanced", "Vyvážené"),
        ("Optimize reaction time", "Optimalizované pre čas odozvy"),
        ("Custom", ""),
        ("Show remote cursor", "Zobrazovať vzdialený ukazovateľ myši"),
        ("Show quality monitor", ""),
        ("Disable clipboard", "Vypnúť schránku"),
        ("Lock after session end", "Po skončení uzamknúť plochu"),
        ("Insert", "Vložiť"),
        ("Insert Lock", "Uzamknúť"),
        ("Refresh", "Aktualizovať"),
        ("ID does not exist", "ID neexistuje"),
        ("Failed to connect to rendezvous server", "Nepodarilo sa pripojiť k zoznamovaciemu serveru"),
        ("Please try later", "Vyskúšajte neskôr"),
        ("Remote desktop is offline", "Vzdialená plocha je pripojená"),
        ("Key mismatch", "Kľúče sa nezhodujú"),
        ("Timeout", "Čas pre nadviazanie pripojenia vypršal"),
        ("Failed to connect to relay server", "Nepodarilo sa pripojiť k prepojovaciemu serveru"),
        ("Failed to connect via rendezvous server", "Nepodarilo sa pripojiť cez zoznamovací server"),
        ("Failed to connect via relay server", "Nepodarilo sa pripojiť cez prepojovací server"),
        ("Failed to make direct connection to remote desktop", "Nepodarilo sa nadviazať priamu komunikáciu so vzdialenou plochou"),
        ("Set Password", "Nastaviť heslo"),
        ("OS Password", "Heslo do operačného systému"),
        ("install_tip", "V niektorých prípadoch RustDesk nefunguje správne z dôvodu riadenia užívateľských oprávnení (UAC). Vyhnete sa tomu kliknutím na nižšie zobrazene tlačítko a nainštalovaním RuskDesk do systému."),
        ("Click to upgrade", "Kliknutím nainštalujete aktualizáciu"),
        ("Click to download", "Kliknutím potvrďte stiahnutie"),
        ("Click to update", "Kliknutím aktualizovať"),
        ("Configure", "Nastaviť"),
        ("config_acc", "Aby bolo možné na diaľku ovládať vašu plochu, je potrebné aplikácii RustDesk udeliť práva \"Dostupnosť\"."),
        ("config_screen", "Aby bolo možné na diaľku sledovať vašu obrazovku, je potrebné aplikácii RustDesk udeliť práva \"Zachytávanie obsahu obrazovky\"."),
        ("Installing ...", "Inštaluje sa"),
        ("Install", "Inštalovať"),
        ("Installation", "Inštalácia"),
        ("Installation Path", "Inštalačný adresár"),
        ("Create start menu shortcuts", "Vytvoriť zástupcu do ponuky Štart"),
        ("Create desktop icon", "Vytvoriť zástupcu na plochu"),
        ("agreement_tip", "Spustením inštalácie prijímate licenčné podmienky."),
        ("Accept and Install", "Prijať a inštalovať"),
        ("End-user license agreement", "Licenčné podmienky dohodnuté s koncovým užívateľom"),
        ("Generating ...", "Generujem ..."),
        ("Your installation is lower version.", "Vaša inštalácia je staršia"),
        ("not_close_tcp_tip", "Nezatvárajte toto okno po celý čas, kedy používate TCP tunel"),
        ("Listening ...", "Čakám na pripojenie ..."),
        ("Remote Host", "Vzdialený počítač"),
        ("Remote Port", "Vzdialený port"),
        ("Action", "Akcia"),
        ("Add", "Pridať"),
        ("Local Port", "Lokálny port"),
        ("Local Address", ""),
        ("Change Local Port", ""),
        ("setup_server_tip", "Pre zrýchlenie pripojenia si nainštalujte svoj vlastný server"),
        ("Too short, at least 6 characters.", "Príliš krátke, vyžaduje sa aspoň 6 znakov."),
        ("The confirmation is not identical.", "Potvrdenie nie je zhodné."),
        ("Permissions", "Práva"),
        ("Accept", "Prijať"),
        ("Dismiss", "Odmietnuť"),
        ("Disconnect", "Odpojiť"),
        ("Allow using keyboard and mouse", "Povoliť používanie klávesnice a myši"),
        ("Allow using clipboard", "Povoliť používanie schránky"),
        ("Allow hearing sound", "Povoliť zvuky"),
        ("Allow file copy and paste", "Povoliť kopírovanie a vkladanie súborov"),
        ("Connected", "Pripojené"),
        ("Direct and encrypted connection", "Priame a šifrované spojenie"),
        ("Relayed and encrypted connection", "Sprostredkované a šifrované spojenie"),
        ("Direct and unencrypted connection", "Priame a nešifrované spojenie"),
        ("Relayed and unencrypted connection", "Sprostredkované a nešifrované spojenie"),
        ("Enter Remote ID", "Zadajte ID vzdialenej plochy"),
        ("Enter your password", "Zadajte svoje heslo"),
        ("Logging in...", "Prihlasovanie sa...."),
        ("Enable RDP session sharing", "Povoliť zdieľanie RDP relácie"),
        ("Auto Login", "Automatické prihlásenie"),
        ("Enable Direct IP Access", "Povoliť priame pripojenie cez IP"),
        ("Rename", "Premenovať"),
        ("Space", "Medzera"),
        ("Create Desktop Shortcut", "Vytvoriť zástupcu na ploche"),
        ("Change Path", "Zmeniť adresár"),
        ("Create Folder", "Vytvoriť adresár"),
        ("Please enter the folder name", "Zadajte názov adresára"),
        ("Fix it", "Opraviť to"),
        ("Warning", "Upozornenie"),
        ("Login screen using Wayland is not supported", "Prihlasovacia obrazovka prostredníctvom Wayland nie je podporovaná"),
        ("Reboot required", "Vyžaduje sa reštart"),
        ("Unsupported display server ", "Nepodporovaný zobrazovací (display) server"),
        ("x11 expected", "očakáva sa x11"),
        ("Port", ""),
        ("Settings", "Nastavenia"),
        ("Username", "Uživateľské meno"),
        ("Invalid port", "Neplatný port"),
        ("Closed manually by the peer", "Manuálne ukončené opačnou stranou pripojenia"),
        ("Enable remote configuration modification", "Povoliť zmeny konfigurácie zo vzdialeného PC"),
        ("Run without install", "Spustiť bez inštalácie"),
        ("Always connected via relay", "Vždy pripojené cez prepájací server"),
        ("Always connect via relay", "Vždy pripájať cez prepájací server"),
        ("whitelist_tip", "Len vymenované IP adresy majú oprávnenie sa pripojiť k vzdialenej správe"),
        ("Login", "Prihlásenie"),
        ("Remember me", ""),
        ("Logout", "Odhlásenie"),
        ("Tags", "Štítky"),
        ("Search ID", "Hľadať ID"),
        ("Current Wayland display server is not supported", "Zobrazovací (display) server Wayland nie je podporovaný"),
        ("whitelist_sep", "Oddelené čiarkou, bodkočiarkou, medzerou alebo koncom riadku"),
        ("Add ID", "Pridať ID"),
        ("Add Tag", "Pridať štítok"),
        ("Unselect all tags", "Zrušiť výber všetkých štítkov"),
        ("Network error", "Chyba siete"),
        ("Username missed", "Chýba užívateľské meno"),
        ("Password missed", "Chýba heslo"),
        ("Wrong credentials", "Nesprávne prihlasovacie údaje"),
        ("Edit Tag", "Upraviť štítok"),
        ("Unremember Password", "Zabudnúť heslo"),
        ("Favorites", "Obľúbené"),
        ("Add to Favorites", "Pridať medzi obľúbené"),
        ("Remove from Favorites", "Odstrániť z obľúbených"),
        ("Empty", "Prázdne"),
        ("Invalid folder name", "Neplatný názov adresára"),
        ("Socks5 Proxy", "Socks5 Proxy"),
        ("Hostname", "Názov počítača"),
        ("Discovered", "Objavené"),
        ("install_daemon_tip", "Ak chcete, aby sa spúšťal pri štarte systému, musíte nainštalovať systémovú službu."),
        ("Remote ID", "Vzdialené ID"),
        ("Paste", "Vložiť"),
        ("Paste here?", "Vložiť sem?"),
        ("Are you sure to close the connection?", "Ste si istý, že chcete ukončiť spojenie?"),
        ("Download new version", "Stiahnuť novú verziu"),
        ("Touch mode", "Dotykový režim"),
        ("Mouse mode", "Režim ovládania myšou"),
        ("One-Finger Tap", "Klepnutie jedným prstom"),
        ("Left Mouse", "Ľavé tlačidlo myši"),
        ("One-Long Tap", "Jedno dlhé klepnutie"),
        ("Two-Finger Tap", "Klepnutie dvoma prstami"),
        ("Right Mouse", "Pravé tlačidlo myši"),
        ("One-Finger Move", "Presúvanie jedným prstom"),
        ("Double Tap & Move", "Dvojité klepnutie a presun"),
        ("Mouse Drag", "Presun myšou"),
        ("Three-Finger vertically", "Pohyb tromi prstami zvisle"),
        ("Mouse Wheel", "Koliesko myši"),
        ("Two-Finger Move", "Pohyb dvoma prstami"),
        ("Canvas Move", "Pohyb zobrazenia"),
        ("Pinch to Zoom", "Roztiahnutím prstov priblížiť"),
        ("Canvas Zoom", "Priblíženie zobrazenia"),
        ("Reset canvas", "Obnoviť zobrazenie"),
        ("No permission of file transfer", "Prenos súborov nie je povolený"),
        ("Note", "Poznámka"),
        ("Connection", "Pripojenie"),
        ("Share Screen", "Zdielať obrazovku"),
        ("CLOSE", "ZATVORIŤ"),
        ("OPEN", "OTVORIŤ"),
        ("Chat", "Chat"),
        ("Total", "Celkom"),
        ("items", "položiek"),
        ("Selected", "Vybrané"),
        ("Screen Capture", "Snímanie obrazovky"),
        ("Input Control", "Ovládanie vstupných zariadení"),
        ("Audio Capture", "Snímanie zvuku"),
        ("File Connection", "Pripojenie súborov"),
        ("Screen Connection", "Pripojenie obrazu"),
        ("Do you accept?", "Súhlasíte?"),
        ("Open System Setting", "Otvorenie nastavení systému"),
        ("How to get Android input permission?", "Ako v systéme Android povoliť oprávnenie písať zo vstupného zariadenia?"),
        ("android_input_permission_tip1", "Aby bolo možné na diaľku ovládať vašu plochu pomocou myši alebo dotykov, je potrebné aplikácii RustDesk udeliť práva \"Dostupnosť\"."),
        ("android_input_permission_tip2", "Prejdite na stránku nastavení systému, nájdite a vstúpte do [Stiahnuté služby], zapnite [RustDesk Input] službu."),
        ("android_new_connection_tip", "Bola prijatá nová požiadavka na ovládanie vášho zariadenia."),
        ("android_service_will_start_tip", "Zapnutie \"Zachytávanie obsahu obrazovky\" automaticky spistí službu, čo iným zariadeniam umožní požiadať o pripojenie k tomuto zariadeniu."),
        ("android_stop_service_tip", "Zastavenie služby automaticky ukončí všetky naviazané spojenia."),
        ("android_version_audio_tip", "Vaša verzia Androidu neumožňuje zaznamenávanie zvuku. Prejdite na verziu Android 10 alebo vyššiu."),
        ("android_start_service_tip", "Klepnite na [Spustiť službu] alebo OTVORTE oprávnenie [Zachytávanie obsahu obrazovky], aby sa aktivovala služba zdieľania obrazovky."),
        ("Account", ""),
        ("Overwrite", "Prepísať"),
        ("This file exists, skip or overwrite this file?", "Preskočiť alebo prepísať existujúci súbor?"),
        ("Quit", "Ukončiť"),
        ("doc_mac_permission", "https://rustdesk.com/docs/en/manual/mac/#enable-permissions"),
        ("Help", "Nápoveda"),
        ("Failed", "Nepodarilo sa"),
        ("Succeeded", "Podarilo sa"),
        ("Someone turns on privacy mode, exit", "Niekto zapne režim súkromia, ukončite ho"),
        ("Unsupported", "Nepodporované"),
        ("Peer denied", "Peer poprel"),
        ("Please install plugins", "Nainštalujte si prosím pluginy"),
        ("Peer exit", "Peer exit"),
        ("Failed to turn off", "Nepodarilo sa vypnúť"),
        ("Turned off", "Vypnutý"),
        ("In privacy mode", "V režime súkromia"),
        ("Out privacy mode", "Mimo režimu súkromia"),
        ("Language", ""),
        ("Keep RustDesk background service", ""),
        ("Ignore Battery Optimizations", ""),
        ("android_open_battery_optimizations_tip", ""),
        ("Connection not allowed", ""),
        ("Legacy mode", ""),
        ("Map mode", ""),
        ("Translate mode", ""),
        ("Use permanent password", ""),
        ("Use both passwords", ""),
        ("Set permanent password", ""),
        ("Enable Remote Restart", ""),
        ("Allow remote restart", ""),
        ("Restart Remote Device", ""),
        ("Are you sure you want to restart", ""),
        ("Restarting Remote Device", ""),
        ("remote_restarting_tip", ""),
        ("Copied", ""),
        ("Exit Fullscreen", "Ukončiť celú obrazovku"),
        ("Fullscreen", "Celá obrazovka"),
        ("Mobile Actions", "Mobilné akcie"),
        ("Select Monitor", "Vyberte možnosť Monitor"),
        ("Control Actions", "Kontrolné akcie"),
        ("Display Settings", "Nastavenia displeja"),
        ("Ratio", "Pomer"),
        ("Image Quality", "Kvalita obrazu"),
        ("Scroll Style", "Štýl posúvania"),
        ("Show Menubar", "Zobraziť panel s ponukami"),
        ("Hide Menubar", "skryť panel s ponukami"),
        ("Direct Connection", "Priame pripojenie"),
        ("Relay Connection", "Reléové pripojenie"),
        ("Secure Connection", "Zabezpečené pripojenie"),
        ("Insecure Connection", "Nezabezpečené pripojenie"),
        ("Scale original", "Pôvodná mierka"),
        ("Scale adaptive", "Prispôsobivá mierka"),
        ("General", ""),
        ("Security", ""),
        ("Theme", ""),
        ("Dark Theme", ""),
        ("Dark", ""),
        ("Light", ""),
        ("Follow System", ""),
        ("Enable hardware codec", ""),
        ("Unlock Security Settings", ""),
        ("Enable Audio", ""),
        ("Unlock Network Settings", ""),
        ("Server", ""),
        ("Direct IP Access", ""),
        ("Proxy", ""),
        ("Apply", ""),
        ("Disconnect all devices?", ""),
        ("Clear", ""),
        ("Audio Input Device", ""),
        ("Deny remote access", ""),
        ("Use IP Whitelisting", ""),
        ("Network", ""),
        ("Enable RDP", ""),
        ("Pin menubar", "Pripnúť panel s ponukami"),
        ("Unpin menubar", "Uvoľniť panel s ponukami"),
        ("Recording", ""),
        ("Directory", ""),
        ("Automatically record incoming sessions", ""),
        ("Change", ""),
        ("Start session recording", ""),
        ("Stop session recording", ""),
        ("Enable Recording Session", ""),
        ("Allow recording session", ""),
        ("Enable LAN Discovery", ""),
        ("Deny LAN Discovery", ""),
        ("Write a message", ""),
        ("Prompt", ""),
        ("Please wait for confirmation of UAC...", ""),
        ("elevated_foreground_window_tip", ""),
        ("Disconnected", ""),
        ("Other", ""),
        ("Confirm before closing multiple tabs", ""),
        ("Keyboard Settings", ""),
        ("Full Access", ""),
        ("Screen Share", ""),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland vyžaduje Ubuntu 21.04 alebo vyššiu verziu."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Wayland vyžaduje vyššiu verziu linuxovej distribúcie. Skúste X11 desktop alebo zmeňte OS."),
        ("JumpLink", "View"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Vyberte obrazovku, ktorú chcete zdieľať (Ovládajte na strane partnera)."),
        ("Show RustDesk", ""),
        ("This PC", ""),
        ("or", ""),
        ("Continue with", ""),
        ("Elevate", ""),
        ("Zoom cursor", ""),
        ("Accept sessions via password", ""),
        ("Accept sessions via click", ""),
        ("Accept sessions via both", ""),
        ("Please wait for the remote side to accept your session request...", ""),
        ("One-time Password", ""),
        ("Use one-time password", ""),
        ("One-time password length", ""),
        ("Request access to your device", ""),
        ("Hide connection management window", ""),
        ("hide_cm_tip", ""),
        ("wayland_experiment_tip", ""),
        ("Right click to select tabs", ""),
        ("Skipped", ""),
        ("Add to Address Book", ""),
        ("Group", ""),
        ("Search", ""),
        ("Closed manually by the web console", ""),
        ("Local keyboard type", ""),
        ("Select local keyboard type", ""),
    ].iter().cloned().collect();
}
