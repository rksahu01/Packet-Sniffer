// EXTRA NEEDED CHARACTERS: Б

use iced::widget::Text;

use crate::translations::types::language::Language;
use crate::StyleType;

pub fn choose_adapters_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Select network adapter to inspect",
        Language::IT => "Seleziona la scheda di rete da ispezionare",
        Language::FR => "Sélectionnez une carte réseau à inspecter",
        Language::ES => "Seleccione el adaptador de red que desea inspeccionar",
        Language::PL => "Wybierz adapter sieciowy do inspekcji",
        Language::DE => "Wähle einen Netzwerkadapter zum überwachen aus",
        Language::UK => "Вибрати мережевий адаптер для інспекції",
        Language::ZH => "选择需要监控的网络适配器",
        Language::RO => "Selectați adaptor de rețea pentru a inspecta",
        Language::KO => "검사할 네트워크 어댑터 선택",
        Language::TR => "İncelemek için bir ağ adaptörü seçiniz",
        Language::RU => "Выберите сетевой адаптер для инспекции",
        Language::PT => "Selecione o adaptador de rede a inspecionar",
        Language::EL => "Επίλεξε τον προσαρμογέα δικτύου για επιθεώρηση",
        // Language::FA => "مبدل شبکه را برای بازرسی انتخاب کنید",
        Language::SV => "Välj nätverksadapter att inspektera",
        Language::FI => "Valitse tarkasteltava verkkosovitin",
        Language::JA => "使用するネットワーク アダプターを選択してください",
        Language::UZ => "Tekshirish uchun tarmoq adapterini tanlang",
        Language::VI => "Hãy chọn network adapter để quan sát",
    })
}

pub fn select_filters_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Select filters to be applied on network traffic",
        Language::IT => "Seleziona i filtri da applicare al traffico di rete",
        Language::FR => "Sélectionnez les filtres à appliquer sur le traffic réseau",
        Language::ES => "Seleccionar los filtros que se aplicarán al tráfico de red",
        Language::PL => "Wybierz filtry, które mają być zastosowane na ruchu sieciowym",
        Language::DE => "Wähle die Filter, die auf den Netzwerkverkehr angewendet werden sollen",
        Language::UK => "Вибрати фільтри, які мають бути застосовані до мережевого трафіку",
        Language::ZH => "选择需要监控的目标",
        Language::RO => "Selectați filtre pentru traficul de rețea",
        Language::KO => "네트워크 트레픽에 적용할 필터 선택",
        Language::TR => "Ağ trafiğine uygulanacak filtreleri seçiniz",
        Language::RU => "Выберите фильтры для применения к сетевому трафику",
        Language::PT => "Selecione os filtros a serem aplicados no tráfego de rede",
        Language::EL => "Επίλεξε τα φίλτρα για εφαρμογή στην κίνηση του δικτύου",
        // Language::FA => "صافی ها را جهت اعمال بر آمد و شد شبکه انتخاب کنید",
        Language::SV => "Välj filtren som ska appliceras på nätverkstrafiken",
        Language::FI => "Valitse suodattimet verkkoliikenteelle",
        Language::JA => "トラフィックに適用するフィルターを選択してください",
        Language::UZ => "Tarmoq trafigiga qo'llaniladigan filtrlarni tanlang",
        Language::VI => "Hãy chọn bộ lọc cho lưu lượng mạng",
    })
}

pub fn start_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::DE | Language::RO | Language::KO => "Start!",
        Language::IT => "Avvia!",
        Language::FR => "Commencer!",
        Language::ES => "¡Empieza!",
        Language::PL => "Rozpocznij!",
        Language::UK => "Почати!",
        Language::ZH => "开始!",
        Language::TR => "Başla!",
        Language::RU => "Начать!",
        Language::PT => "Começar!",
        Language::EL => "Ξεκίνα!",
        // Language::FA => "شروع!",
        Language::SV => "Starta!",
        Language::FI => "Aloita!",
        Language::JA => "開始！",
        Language::UZ => "Boshlash!",
        Language::VI => "Bắt đầu!",
    }
}

pub fn address_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Address",
        Language::IT => "Indirizzo",
        Language::FR | Language::DE => "Adresse",
        Language::ES => "Dirección",
        Language::PL | Language::TR => "Adres",
        Language::UK => "Адреса",
        Language::ZH => "网络地址",
        Language::RO => "Adresă",
        Language::KO => "주소",
        Language::RU => "Адрес",
        Language::PT => "Endereço",
        Language::EL => "Διεύθυνση",
        // Language::FA => "نشانی",
        Language::SV => "Adress",
        Language::FI => "Osoite",
        Language::JA => "アドレス",
        Language::UZ => "Manzil",
        Language::VI => "Địa chỉ",
    }
}

pub fn addresses_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Addresses",
        Language::IT => "Indirizzi",
        Language::FR => "Adresses",
        Language::ES => "Direcciones",
        Language::PL => "Adresy",
        Language::DE => "Adressen",
        Language::UK => "Адреси",
        Language::ZH => "网络地址",
        Language::RO => "Adrese",
        Language::KO => "주소",
        Language::TR => "Adresler",
        Language::RU => "Адреса",
        Language::PT => "Endereços",
        Language::EL => "Διευθύνσεις",
        // Language::FA => "نشانی ها",
        Language::SV => "Adresser",
        Language::FI => "Osoitteet",
        Language::JA => "アドレス",
        Language::UZ => "Manzillar",
        Language::VI => "Danh sách địa chỉ",
    }
}

pub fn ip_version_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "IP version",
        Language::IT => "Versione IP",
        Language::FR => "Version IP",
        Language::ES => "Versión IP",
        Language::PL => "Wersja IP",
        Language::DE => "IP Version",
        Language::UK => "Версія IP",
        Language::ZH => "目标IP协议版本",
        Language::RO => "Versiune IP",
        Language::KO => "IP 버전",
        Language::TR => "IP versiyonu",
        Language::RU => "Версия IP",
        Language::PT => "Versão de IP",
        Language::EL => "Έκδοση IP",
        // Language::FA => "نسخهٔ IP",
        Language::SV => "IP-version",
        Language::FI => "IP-versio",
        Language::JA => "IP バージョン",
        Language::UZ => "IP versiyasi",
        Language::VI => "Phiên bản IP",
    }
}

pub fn protocol_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::RO => "Protocol",
        Language::IT => "Protocollo",
        Language::FR => "Protocole",
        Language::ES | Language::PT => "Protocolo",
        Language::PL => "Protokół",
        Language::DE | Language::SV => "Protokoll",
        Language::UK | Language::RU => "Протокол",
        Language::ZH => "协议",
        Language::KO => "프로토콜",
        Language::TR => "Protokolü",
        Language::EL => "Πρωτόκολλο",
        // Language::FA => "پیوندنامهٔ",
        Language::FI => "Protokolla",
        Language::JA => "プロトコル",
        Language::UZ => "Protokoli",
        Language::VI => "Phương thức",
    }
}

pub fn traffic_rate_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Traffic rate",
        Language::IT => "Intensità del traffico",
        Language::FR => "Fréquence du traffic",
        Language::ES => "Tasa de tráfico",
        Language::PL => "Prędkość ruchu",
        Language::DE => "Daten Frequenz",
        Language::UK => "Швидкість руху",
        Language::ZH => "网络速率图",
        Language::RO => "Rata de trafic",
        Language::KO => "트레픽 속도",
        Language::TR => "Trafik oranı",
        Language::RU => "Cкорость трафика",
        Language::PT => "Taxa de tráfego",
        Language::EL => "Ρυθμός κίνησης",
        // Language::FA => "نرخ آمد و شد",
        Language::SV => "Datafrekvens",
        Language::FI => "Liikennemäärä",
        Language::JA => "トラフィック レート",
        Language::UZ => "Trafik tezligi",
        Language::VI => "Lưu lượng truy cập",
    })
}

pub fn settings_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Settings",
        Language::IT => "Impostazioni",
        Language::FR => "Paramètres",
        Language::ES => "Ajustes",
        Language::PL => "Ustawienia",
        Language::DE => "Einstellungen",
        Language::UK => "Налаштування",
        Language::ZH => "设置",
        Language::RO => "Setări",
        Language::KO => "설정",
        Language::TR => "Ayarlar",
        Language::RU => "Настройки",
        Language::PT => "Configurações",
        Language::EL => "Ρυθμίσεις",
        // Language::FA => "پیکربندی",
        Language::SV => "Inställningar",
        Language::FI => "Asetukset",
        Language::JA => "設定",
        Language::UZ => "Sozlamalar",
        Language::VI => "Cài đặt",
    }
}

pub fn yes_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Yes",
        Language::IT => "Sì",
        Language::FR => "Oui",
        Language::ES => "Sí",
        Language::PL => "Tak",
        Language::DE | Language::SV => "Ja",
        Language::UK => "Так",
        Language::ZH => "是",
        Language::RO => "Da",
        Language::KO => "네",
        Language::TR => "Evet",
        Language::RU => "Да",
        Language::PT => "Sim",
        Language::EL => "Ναι",
        // Language::FA => "بله",
        Language::FI => "Kyllä",
        Language::JA => "はい",
        Language::UZ => "Ha",
        Language::VI => "Chấp nhận",
    })
}

pub fn ask_quit_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Are you sure you want to quit this analysis?",
        Language::IT => "Sei sicuro di voler interrompere questa analisi?",
        Language::FR => "Êtes-vous sûr de vouloir quitter l'application ?",
        Language::ES => "¿Estás seguro de que quieres dejar este análisis?",
        Language::PL => "Jesteś pewien, że chcesz zakończyć analizę?",
        Language::DE => "Bist du sicher, dass du diese Analyse beenden willst?",
        Language::UK => "Чи справді хочеш закінчити аналіз?",
        Language::ZH => "您确定退出当前监控吗?",
        Language::RO => "Sunteți sigur că doriți să renunțați la această analiză?",
        Language::KO => "정말로 분석을 종료하겠습니까?",
        Language::TR => "Bu analizden çıkmak istediğine emin misin?",
        Language::RU => "Вы уверены, что хотите выйти из текущего анализа?",
        Language::PT => "Tem a certeza que deseja sair desta análise?",
        Language::EL => "Είσαι σίγουρος ότι θες να κλείσεις την ανάλυση;",
        // Language::FA => "آیا مطمئن هستید می خواهید از این تحلیل خارج شوید؟",
        Language::SV => "Är du säker på att du vill avsluta analysen?",
        Language::FI => "Haluatko varmasti lopettaa analyysin?",
        Language::JA => "分析を終了しますか？",
        Language::UZ => "Tahlildan chiqishga ishonchingiz komilmi?",
        Language::VI => "Bạn có chắc là muốn thoát phiên phân tích này?",
    })
}

pub fn quit_analysis_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Quit analysis",
        Language::IT => "Interrompi analisi",
        Language::FR => "Quitter l'analyse",
        Language::ES => "Quitar el análisis",
        Language::PL => "Zakończ analize",
        Language::DE => "Analyse beenden",
        Language::UK => "Закінчити аналіз",
        Language::ZH => "退出监控",
        Language::RO => "Renunță la analiză",
        Language::KO => "분석종료",
        Language::TR => "Analizden çık",
        Language::RU => "Закончить анализ",
        Language::PT => "Sair da análise",
        Language::EL => "Έξοδος ανάλυσης",
        // Language::FA => "خروج از تحلیل",
        Language::SV => "Avsluta analys",
        Language::FI => "Lopeta analyysi",
        Language::JA => "分析の終了",
        Language::UZ => "Tahlildan chiqish",
        Language::VI => "Thoát phiên phân tích",
    }
}

pub fn ask_clear_all_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Are you sure you want to clear notifications?",
        Language::IT => "Sei sicuro di voler eliminare le notifiche?",
        Language::FR => "Êtes-vous sûr de vouloir effacer les notifications ?",
        Language::ES => "¿Seguro que quieres borrar las notificaciones?",
        Language::PL => "Czy na pewno chcesz wyczyścić powiadomienia?",
        Language::DE => "Bist du sicher, dass du alle Benachrichtigungen löschen willst?",
        Language::UK => "Чи справді хочеш видалити всі повідомлення?",
        Language::ZH => "确定清除所有通知?",
        Language::RO => "Sigur doriți să ștergeți notificările?",
        Language::KO => "알림을 삭제하시겠습니까?",
        Language::TR => "Bildirimleri temizlemek istediğine emin misin?",
        Language::RU => "Вы уверены, что хотите удлить все уведомления?",
        Language::PT => "Tem a certeza que deseja eliminar as notificações?",
        Language::EL => "Είσαι σίγουρος ότι θες να κάνεις εκκαθάριση των ειδοποιήσεων;",
        // Language::FA => "آیا مطمئن هستید می خواهید اعلان ها را پاک کنید؟",
        Language::SV => "Är du säker på att du vill radera notifikationerna?",
        Language::FI => "Haluatko varmasti tyhjentää ilmoitukset?",
        Language::JA => "すべての通知を削除します。よろしいですか？",
        Language::UZ => "Haqiqatan ham bildirishnomalarni tozalamoqchimisiz?",
        Language::VI => "Bạn có chắc là muốn xóa các thông báo?",
    })
}

pub fn clear_all_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Clear all",
        Language::IT => "Elimina tutto",
        Language::FR => "Tout effacer",
        Language::ES => "Borrar todo",
        Language::PL => "Wyczyść wszystko",
        Language::DE => "Alle löschen",
        Language::UK => "Видалити все",
        Language::ZH => "清除所有",
        Language::RO => "Ștergeți tot",
        Language::KO => "모두 지우기",
        Language::TR => "Hepsini temizle",
        Language::RU => "Очистить всё",
        Language::PT => "Limpar tudo",
        Language::EL => "Εκκαθάριση όλων",
        // Language::FA => "پاک کردن همه",
        Language::SV => "Radera alla",
        Language::FI => "Tyhjennä kaikki",
        Language::JA => "すべて削除",
        Language::UZ => "Barchasini tozalash",
        Language::VI => "Xóa tất cả",
    }
}

pub fn hide_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Hide",
        Language::IT => "Nascondi",
        Language::FR => "Masquer",
        Language::ES => "Ocultar",
        Language::PL => "Ukryj",
        Language::DE => "Verstecken",
        Language::UK => "Заховати",
        Language::ZH => "隐藏",
        Language::RO => "Ascundeți",
        Language::KO => "숨기기",
        Language::TR => "Gizle",
        Language::RU => "Скрыть",
        Language::PT => "Esconder",
        Language::EL => "Κλείσιμο",
        // Language::FA => "پنهان کردن",
        Language::SV => "Göm",
        Language::FI => "Piilota",
        Language::JA => "隠す",
        Language::UZ => "Yashirish",
        Language::VI => "Ẩn",
    }
}

pub fn network_adapter_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::VI => "Network adapter",
        Language::IT => "Adattatore di rete",
        Language::FR => "Carte réseau",
        Language::ES => "Adaptador de red",
        Language::PL => "Adapter sieciowy",
        Language::DE => "Netzwerkadapter",
        Language::UK => "Мережквий адаптер",
        Language::ZH => "网络适配器",
        Language::RO => "Adaptor de rețea",
        Language::KO => "네트워크 어뎁터",
        Language::TR => "Ağ adaptörü",
        Language::RU => "Сетевой интерфейс",
        Language::PT => "Adaptador de rede",
        Language::EL => "Προσαρμογέας δικτύου",
        // Language::FA => "مبدل شبکه",
        Language::SV => "Nätverksadapter",
        Language::FI => "Verkkosovitin",
        Language::JA => "ネットワーク アダプター",
        Language::UZ => "Tarmoq adapteri",
    }
}

pub fn no_addresses_translation(language: Language, adapter: &str) -> Text<'static, StyleType> {
    let network_adapter_translation = network_adapter_translation(language);
    Text::new(match language {
        Language::EN => format!("No traffic can be observed because the adapter you selected has no active addresses...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 If you are sure you are connected to the internet, try choosing a different adapter."),
        Language::IT => format!("Non è osservabile alcun traffico perché l'adattatore di rete selezionato non ha indirizzi attivi...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Se sei sicuro di essere connesso ad internet, prova a scegliere un adattatore diverso."),
        Language::FR => format!("Aucun trafic ne peut être observé, car la carte réseau que vous avez saisie n'a pas d'adresse...\n\n\
                                {network_adapter_translation} : {adapter}\n\n\
                                Si vous êtes sûr d'être connecté à internet, essayez une autre carte."),
        Language::ES => format!("No se puede observar ningún tráfico porque el adaptador seleccionado no tiene direcciones activas...\n\n\
                                 {network_adapter_translation} : {adapter}\n\n\
                                 Si estás seguro de que estás conectado a Internet, prueba a elegir otro adaptador."),
        Language::PL => format!("Nie można zaobserwować żadnego ruchu, ponieważ wybrany adapter nie ma aktywnych adresów...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Jeśli jesteś pewien, że jesteś podłączony do internetu, spróbuj wybrać inny adapter."),
        Language::DE => format!("Es kann kein Netzwerkverkehr beobachtet werden, weil der Adapter keine aktiven Adressen hat...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Wenn du dir sicher bist, dass du mit dem Internet verbunden bist, probier einen anderen Adapter auszuwählen."),
        Language::UK => format!("Не зафіксовано жодного мережевого трафіку тому що вибраний адаптер немає активних адрес... \n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Якщо Ти впевнений, що підключений до інтернету, спробуй вибрати інший адаптер."),
        Language::ZH => format!("您选择的网络适配器当前无活动网络...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                如果您确信您已成功连接互联网, 请尝试选择其他网络适配器."),
        Language::RO => format!("Niciun trafic nu poate fi observat deoarece adaptorul selectat nu are adrese active...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Dacă sunteți sigur că sunteți conectat la internet, încercați să alegeți un alt adaptor."),
        Language::KO => format!("선택한 어댑터에 유효한 주소가 없기 때문에 트래픽을 확인할 수 없습니다...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                인터넷이 연결되어있다면 다른 어댑터로 시도해보세요."),
        Language::TR => format!("Seçtiğiniz adaptör aktif bir adrese sahip olmadığı için hiç bir trafik izlenemez...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Eğer gerçekten internete bağlı olduğunuza eminseniz, başka bir adaptör seçmeyi deneyiniz."),
        Language::RU => format!("Наблюдение за трафиком не возможно, потому что Вы выбрали интерфейс без активного адреса...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Если Вы уверены, что подключены к Интернету, попробуйте выбрать другой интерфейс."),
        Language::PT => format!("Não é possível observar tráfego porque o adaptador que selecionou não tem endereços ativos...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Se tiver a certeza que está ligado à internet, tente escolher um adaptador diferente."),
        Language::EL => format!("Δεν μπορεί να ανιχνευθεί κίνηση επειδή ο προσαρμογέας που επέλεξες δεν έχει ενεργές διευθύνσεις...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Αν είσαι σίγουρος ότι είσαι συνδεδεμένος στο διαδίκτυο, δοκίμασε αν επιλέξεις έναν διαφορετικό προσαρμογέα."),
        // Language::FA => format!("هیچ آمد و شدی قابل مشاهده نیست چون مبدلی که انتخاب کرده اید هیچ نشانی فعالی ندارد...\n\n\
        //                         مبدل شبکه: {adapter}\n\n\
        //                         اگر مطمئن هستید به اینترنت وصل هستید، سعی کنید مبدل متفاوتی را انتخاب کنید."),
        Language::SV => format!("Det går inte att observa någon trafik eftersom den valda adaptern inte har några aktiva adresser ...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Om du är säker att du är ansluten till internet, testa att välja en annan adapter."),
        Language::FI => format!("Liikennettä ei voitu havainnoida, koska valitulla sovittimella ei ole aktiivista osoitetta...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Jos olet varma että sinulla on internet-yhteys, kokeile valita toinen verkkosovitin."),
        Language::JA => format!("選択されたアダプターが有効なアドレスを持っていないため、トラフィックを観測できていません...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                インターネットに接続しているか確認し、別のネットワーク アダプターを試してください。"),
        Language::UZ => format!("Trafik kuzatilmaydi, chunki siz tanlagan adapterda faol manzillar yo'q...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Internetga ulanganingizga ishonchingiz komil bo'lsa, boshqa adapterni tanlashga harakat qiling"),
        Language::VI => format!("Không thể quan sát lưu lượng nào vì adapter mà bạn chọn không địa chỉ hoạt động...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Nếu bạn đã chắc chắn kết nối với internet, hãy thử chọn network adapter khác."),
    })
}

pub fn waiting_translation(language: Language, adapter: &str) -> Text<'static, StyleType> {
    let network_adapter_translation = network_adapter_translation(language);
    Text::new(match language {
        Language::EN => format!("No traffic has been observed yet. Waiting for network packets...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Are you sure you are connected to the internet and you have selected the correct adapter?"),
        Language::IT => format!("Nessun tipo di traffico è stato osservato finora. Attendo pacchetti di rete...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Sei sicuro di esser connesso ad internet e di aver selezionato l'adattatore corretto?"),
        Language::FR => format!("Aucun trafic n'a été capturé pour le moment. En attente de paquets...\n\n\
                                {network_adapter_translation} : {adapter}\n\n\
                                Êtes-vous sûr d'être connecté à internet et d'avoir selectionné la bonne carte réseau ?"),
        Language::ES => format!("Aún no se ha captado tráfico. Esperando paquetes...\n\n\
                                 {network_adapter_translation} : {adapter}\n\n\
                                 ¿Está seguro de que está conectado a Internet y ha seleccionado la tarjeta de red correcta?"),
        Language::PL => format!("Nie zaobserowano żadnego ruchu sieciowego. Oczekiwanie na pakiety...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Czy na pewno jesteś podłączony do internetu i wybrałeś właściwy adapter?"),
        Language::DE => format!("Noch kein Netzwerkverkehr beobachtet. Warten auf Pakete...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Bist du sicher, dass du mit dem Internet verbunden bist und den richtigen Adapter ausgewählt hast?"),
        Language::UK => format!("Не зафіксовано жодного мережевого трафіку. Очікування на пакети...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Чи Ти дійсно підключений до інтернету і вибрав відповідний мережевий адаптер?"),
        Language::ZH => format!("暂无流量数据. 等待网络活动中......\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 您确信您已成功连接到互联网, 并选择了当前正在使用的的网络适配器吗?"),
        Language::RO => format!("Nu a fost observat încă trafic. Se așteaptă pachetele de rețea...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Ești sigur că ești conectat la internet și ai selectat adaptorul corect?"),
        Language::KO => format!("아직 트래픽이 관찰되지 않았습니다. 네트워크 패킷 대기 중...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                인터넷에 연결되어 있고 올바른 어댑터를 선택하셨습니까?"),
        Language::TR => format!("Henüz bir trafik algılanamadı. Ağ paketleri için bekleniyor...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 İnternete bağlı olduğunuza ve doğru adaptörü seçtiğinize emin misiniz?"),
        Language::RU => format!("Трафик не обнаружен. Ожидаем сетевые пакеты...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Вы уверены, что подключены к Интернету и выбрали правильный интерфейс?"),
        Language::PT => format!("Ainda não foi observado tráfego. Aguardando por pacotes...\n\n\
                                {network_adapter_translation}: {adapter}\n\n\
                                Tem a certeza de que está ligado à internet e selecionou o adaptador correto?"),
        Language::EL => format!("Δεν έχει παρατηρηθεί κίνηση μέχρι στιγμής. Ανέμενε για πακέτα δικτύου...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Είσαι σίγουρος ότι είσαι συνδεδεμένος στο διαδίκτυο και ότι έχεις επιλέξει τον σωστό προσαρμογέα;"),
        // Language::FA => format!("هنوز هیچ آمد و شدی مشاهده نشده است. در حال انتظار برای بسته های شبکه...\n\n
        //                         مبدل شبکه: {adapter}\n\n
        //                         آیا مطمئن هستید به اینترنت وصل هستید و مبدل درست را انتخاب کرده اید؟"),
        Language::SV => format!("Ingen trafik har observerats ännu. Väntar på paket ...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Är du säker på att du är ansluten till internet och att du har valt rätt adapter?"),
        Language::FI => format!("Ei vielä havaittua liikennettä. Odotetaan verkkopaketteja...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Onhan sinulla varmasti internet-yhteys ja olet valinnut oikean verkkosovittimen."),
        Language::JA => format!("トラフィックがまだ観測できていません。ネットワーク パケットを待っています...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 インターネットに接続していて、正しいアダプターを選択していますか?"),
        Language::UZ => format!(
            "Hali hech qanday trafik aniqlanmadi. Tarmoq paketlari kutilmoqda...\n\n\
            {network_adapter_translation}: {adapter}\n\n\
            Internetga ulanganingizga va to'g'ri adapterni tanlaganingizga ishonchingiz komilmi?"),
        Language::VI => format!("Chưa có lưu lượng để quan sát. Đang đợi các gói tin...\n\n\
                                 {network_adapter_translation}: {adapter}\n\n\
                                 Bạn có chắc là đã kết nối với internet và đã chọn đúng network adapter?"),
    })
}

pub fn some_observed_translation(language: Language, observed: u128) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => format!("Total intercepted packets: {observed}\n\n\
                                 Filtered packets: 0\n\n\
                                 Some packets have been intercepted, but still none has been selected according to the filters you specified..."),
        Language::IT => format!("Totale pacchetti intercettati: {observed}\n\n\
                                 Pacchetti filtrati: 0\n\n\
                                 Alcuni pacchetti sono stati intercettati, ma ancora nessuno è stato selezionato secondo i filtri specificati..."),
        Language::FR => format!("Total des paquets interceptés: {observed}\n\n\
                                 Paquets filtrés: 0\n\n\
                                 Certains paquets ont été interceptés, mais aucun ne satisfait les critères des filtres sélectionnés..."),
        Language::ES => format!("Total de paquetes interceptados: {observed}\n\n\
                                 Paquetes filtrados: 0\n\n\
                                 Se interceptaron algunos paquetes, pero ninguno de ellos cumplía los criterios de los filtros seleccionados..."),
        Language::PL => format!("Suma przechwyconych pakietów: {observed}\n\n\
                                 Przefiltrowane pakiety: 0\n\n\
                                 Niektóre pakiety zostały przechwycone, ale żaden nie został wybrany zgodnie z wskazanymi filtrami..."),
        Language::DE => format!("Anzahl der empfangenen Pakete: {observed}\n\n\
                                 Gefilterte Pakete: 0\n\n\
                                 Ein Paar Pakete wurden empfangen, aber es entsprechen noch keine den gewählten Filtern..."),
        Language::UK => format!("Сума перехоплених пакетів: {observed}\n\n\
                                 Відфільтровані пакеті: 0\n\n\
                                 Деякі пакети були перехоплені, але жоден з них не був вибраний відповідно до вказаних фільтрів..."),
        Language::ZH => format!("监测到的数据包总数: {observed}\n\n\
                                 目标数据包总数: 0\n\n\
                                 当前已监测到一些数据包, 但其中并未包含您的目标数据包......"),
        Language::RO => format!("Total pachete interceptate: {observed}\n\n\
                                Pachete filtrate: 0\n\n\
                                Unele pachete au fost interceptate, dar încă niciunul nu a fost selectat conform filtrelor pe care le-ați specificat..."),
        Language::KO => format!("감지한 총 패킷: {observed}\n\n\
                                필터링된 패킷: 0\n\n\
                                일부 패킷이 감지되었지만, 지정한 필터에 따라 선택되지 않았습니다..."),
        Language::TR => format!("Toplam yakalanan paketler: {observed}\n\n\
                                 Filterelenen paketler: 0\n\n\
                                 Bazı paketler yakalandı, fakat belirttiğiniz filtrelere göre hiç biri seçilmedi..."),
        Language::RU => format!("Всего пакетов перехвачено: {observed}\n\n\
                                 Фильтровано пакетов: 0\n\n\
                                 Сетевые пакеты были перехвачены, но ни один из них не соответствует заданным фильтрам..."),
        Language::PT => format!("Total de pacotes interceptados: {observed}\n\n\
                                Pacotes filtrados: 0\n\n\
                                Alguns pacotes foram interceptados, mas nenhum deles foi selecionado de acordo com os filtros especificados..."),
        Language::EL => format!("Συνολικά αναχαιτισμένα πακέτα: {observed}\n\n\
                                 Φιλτραρισμένα πακέτα: 0\n\n\
                                 Κάποια από τα πακέτα έχουν αναχαιτιστεί, αλλά κανένα ακόμη δεν έχει επιλεγεί σύμφωνα με τα φίλτρα που επέλεξες..."),
        // Language::FA => format!("مجموع بسته های رهگیری شده: {observed}\n\n\
        //                         بسته های صاف شده: 0\n\n\
        //                         شماری از بسته ها رهگیری شده اند، ولی هنوز هیچ کدام بر اساس صافی تعیین شده شما انتخاب نشده اند..."),
        Language::SV => format!("Antal fångade paket: {observed}\n\n\
                                 Filtrerade paket: 0\n\n\
                                 Några paket har fångats, men än har inget valts enligt de angivna filtren ..."),
        Language::FI => format!("Siepattuja paketteja yhteensä: {observed}\n\n\
                                 Suodatettuja paketteja: 0\n\n\
                                 Joitakin paketteja on siepattu, mutta yhtäkään ei ole valittu määrittämiesi suodattimien mukaan..."),
        Language::JA => format!("取得したパケット数: {observed}\n\n\
                                 フィルター後のパケット数: 0\n\n\
                                 パケットは取得できていますが、設定されたフィルタリングにより表示されません..."),
        Language::UZ => format!(
            "Jami ushlangan paketlar: {observed}\n\n\
            Filtrlangan paketlar: 0\n\n\
            Tarmoq paketlari ushlandi, lekin ularning hech biri belgilangan filtrlarga mos kelmadi..."),
        Language::VI => format!("Tổng số gói tin bị chặn: {observed}\n\n\
                                 Các gói tin đã lọc: 0\n\n\
                                 Một số gói đã bị chặn, nhưng vẫn chưa có gói tin nào được bắt theo bộ lọc bạn đã chọn..."),
    })
}

pub fn filtered_packets_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Filtered packets",
        Language::IT => "Pacchetti filtrati",
        Language::FR => "Paquets filtrés",
        Language::ES => "Paquetes filtrados",
        Language::PL => "Przefiltrowane pakiety",
        Language::DE => "Gefilterte Pakete",
        Language::UK => "Відфільтровані пакети",
        Language::ZH => "目标数据包计数",
        Language::RO => "Pachete filtrate",
        Language::KO => "필터링된 패킷",
        Language::TR => "Filtrelenen paketler",
        Language::RU => "Отфильтровано пакетов",
        Language::PT => "Pacotes filtrados",
        Language::EL => "Φιλτραρισμένα πακέτα",
        // Language::FA => "بسته های صاف شده",
        Language::SV => "Filtrerade paket",
        Language::FI => "Suodatettuja paketteja",
        Language::JA => "フィルタリングされたパケット",
        Language::UZ => "Filtrlangan paketlar",
        Language::VI => "Các gói tin đã được lọc",
    }
}

pub fn filtered_bytes_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Filtered bytes",
        Language::IT => "Byte filtrati",
        Language::FR => "Octets filtrés",
        Language::ES | Language::PT => "Bytes filtrados",
        Language::PL => "Przechwycone bajty",
        Language::DE => "Gefilterte Bytes",
        Language::UK => "Відфільтровані байти",
        Language::ZH => "目标网络流量计数",
        Language::RO => "Octeți filtrați",
        Language::KO => "필터링된 바이트",
        Language::TR => "Filtrelenen bayt",
        Language::RU => "Отфильтровано байт",
        Language::EL => "Φιλτραρισμένα bytes",
        // Language::FA => "بایت های صاف شده",
        Language::SV => "Filtrerade bytes",
        Language::FI => "Suodatettuja tavuja",
        Language::JA => "フィルタリングされたバイト",
        Language::UZ => "Filtrlangan baytlar",
        Language::VI => "Các bytes đã được lọc",
    }
}

pub fn of_total_translation(language: Language, percentage: &str) -> String {
    match language {
        Language::EN => format!("({percentage} of the total)"),
        Language::IT => format!("({percentage} del totale)"),
        Language::FR => format!("({percentage} du total)"),
        Language::ES => format!("({percentage} del total)"),
        Language::PL => format!("({percentage} z całości)"),
        Language::DE => format!("({percentage} der Gesamtzahl)"),
        Language::UK => {
            format!("({percentage} від загальної суми)")
        }
        Language::ZH => {
            format!("(占所有数据包的 {percentage})")
        }
        Language::RO => {
            format!("({percentage} din total)")
        }
        Language::KO => {
            format!("({percentage} 의 일부)")
        }
        Language::TR => format!("toplamın ({percentage})"),
        Language::RU => {
            format!("({percentage} от общего числа)")
        }
        Language::PT => {
            format!("({percentage} do total)")
        }
        Language::EL => {
            format!("({percentage} από τα συνολικά)")
        }
        // Language::FA => format!("({percentage} از مجموع)"),
        Language::SV => format!("({percentage} av totalen)"),
        Language::FI => format!("({percentage} kokonaismäärästä)"),
        Language::JA => format!("(トータル: {percentage} )"),
        Language::UZ => format!("(Jami: {percentage} )"),
        Language::VI => format!("({percentage} trên tổng cộng)"),
    }
}



pub fn error_translation(language: Language, error: &str) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => format!(
            "An error occurred! \n\n\
                                {error}"
        ),
        Language::IT => format!(
            "Si è verificato un errore! \n\n\
                                {error}"
        ),
        Language::FR => format!(
            "Une erreur est survenue! \n\n\
                                {error}"
        ),
        Language::ES => format!(
            "¡Se ha producido un error! \n\n\
                                {error}"
        ),
        Language::PL => format!(
            "Wystąpił błąd! \n\n\
                                {error}"
        ),
        Language::DE => format!(
            "Es ist ein Fehler aufgetreten! \n\n\
                                {error}"
        ),
        Language::UK => format!(
            "Виступила помилка! \n\n\
                                {error}"
        ),
        Language::ZH => format!(
            "发生了一些错误! \n\n\
                                {error}"
        ),
        Language::RO => format!(
            "A apărut o eroare! \n\n\
                                {error}"
        ),
        Language::KO => format!(
            "오류가 발생하였습니다! \n\n\
                                {error}"
        ),
        Language::TR => format!(
            "Bir hata oluştu! \n\n\
                                 {error}"
        ),
        Language::RU => format!(
            "Произошла ошибка! \n\n\
                                 {error}"
        ),
        Language::PT => format!(
            "Ocorreu um erro! \n\n\
                                {error}"
        ),
        Language::EL => format!(
            "Κάποιο σφάλμα συνέβη! \n\n\
                                {error}"
        ),
        // Language::FA => format!(
        //     "خطایی رخ داد! \n\n\
        //                         {error}"
        // ),
        Language::SV => format!(
            "Ett fel inträffade! \n\n\
                                {error}"
        ),
        Language::FI => format!(
            "Tapahtui virhe! \n\n\
                                {error}"
        ),
        Language::JA => format!(
            "エラーが発生しました! \n\n\
                                {error}"
        ),
        Language::UZ => format!(
            "Xatolik yuz berdi!\n\n\
                                {error}"
        ),
        Language::VI => format!(
            "Đã có lỗi xảy ra! \n\n\
                                {error}"
        ),
    })
}


pub fn packets_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "packets",
        Language::IT => "pacchetti",
        Language::FR => "paquets",
        Language::ES => "paquetes",
        Language::PL => "pakiety",
        Language::DE => "Pakete",
        Language::UK => "пакети",
        Language::ZH => "数据包",
        Language::RO => "pachete",
        Language::KO => "패킷",
        Language::TR | Language::SV => "paket",
        Language::RU => "пакетов",
        Language::PT => "pacotes",
        Language::EL => "πακέτα",
        // Language::FA => "بسته ها",
        Language::FI => "paketit",
        Language::JA => "パケット",
        Language::UZ => "paketlar",
        Language::VI => "các gói tin",
    }
}

pub fn packets_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "packets per second",
        Language::IT => "pacchetti al secondo",
        Language::FR => "paquets par seconde",
        Language::ES => "paquetes por segundo",
        Language::PL => "pakiety na sekundę",
        Language::DE => "Pakete pro Sekunde",
        Language::UK => "пакети на секунду",
        Language::ZH => "数据包",
        Language::RO => "pachete pe secundă",
        Language::KO => "초당 패킷",
        Language::TR => "saniye başı paket",
        Language::RU => "пакетов в секунду",
        Language::PT => "pacotes por segundo",
        Language::EL => "πακέτα ανά δευτερόλεπτο",
        // Language::FA => "بسته در ثانیه",
        Language::SV => "paket per sekund",
        Language::FI => "pakettia sekunnissa",
        Language::JA => "1 秒あたりのパケット数",
        Language::UZ => "paket soniyasiga",
        Language::VI => "gói tin trên giây",
    }
}

pub fn bytes_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::ES | Language::PT | Language::EL | Language::SV | Language::VI => {
            "bytes"
        }
        Language::DE => "Bytes",
        Language::IT => "byte",
        Language::FR => "octets",
        Language::PL => "bajty",
        Language::UK => "байти",
        Language::ZH => "网络流量",
        Language::RO => "octeți",
        Language::KO => "바이트",
        Language::TR | Language::UZ => "bayt",
        Language::RU => "байтов",
        // Language::FA => "بایت ها",
        Language::FI => "tavua",
        Language::JA => "バイト",
    }
}

pub fn bytes_chart_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "bytes per second",
        Language::IT => "byte al secondo",
        Language::FR => "octets par seconde",
        Language::ES | Language::PT => "bytes por segundo",
        Language::PL => "bajty na sekundę",
        Language::DE => "Bytes pro Sekunde",
        Language::UK => "байти на секунду",
        Language::ZH => "网络流量",
        Language::RO => "octeți pe secundă",
        Language::KO => "초당 바이트",
        Language::TR => "saniye başı bayt",
        Language::RU => "байтов в секунду",
        Language::EL => "bytes ανά δευτερόλεπτο",
        // Language::FA => "بایت در ثانیه",
        Language::SV => "bytes per sekund",
        Language::FI => "tavua sekunnissa",
        Language::JA => "1 秒あたりのバイト量",
        Language::UZ => "bayt soniyasiga",
        Language::VI => "byte trên giây",
    }
}

pub fn notifications_title_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Customize your notifications",
        Language::IT => "Personalizza le tue notifiche",
        Language::FR => "Personnalisez vos notifications",
        Language::ES => "Personaliza tus notificaciones",
        Language::PL => "Dostosuj powiadomienia",
        Language::DE => "Stell deine Benachrichtigungen ein",
        Language::UK => "Достосуй повідомлення",
        Language::ZH => "自定义通知",
        Language::RO => "Personalizați-vă notificările",
        Language::KO => "사용자 지정 알림",
        Language::TR => "Bildirimlerinizi özelleştirin",
        Language::RU => "Настройка уведомлений",
        Language::PT => "Personalize as suas notificações",
        Language::EL => "Εξατομίκευση ειδοποιήσεων",
        // Language::FA => "اعلان های خود را سفارشی کنید",
        Language::SV => "Anpassa dina notifikationer",
        Language::FI => "Muokkaa ilmoituksiasi",
        Language::JA => "通知のカスタマイズ",
        Language::UZ => "Bildirishnomalaringizni sozlang",
        Language::VI => "Tùy chỉnh thông báo của bạn",
    })
}

pub fn appearance_title_translation(language: Language) -> Text<'static, StyleType> {
    Text::new(match language {
        Language::EN => "Choose your favorite theme",
        Language::IT => "Scegli il tuo tema preferito",
        Language::FR => "Sélectionnez votre thème préféré",
        Language::ES => "Elige tu tema favorito",
        Language::PL => "Wybierz swój ulubiony motyw",
        Language::DE => "Wähl dein Lieblingsdesign",
        Language::UK => "Вибери улюблену тему",
        Language::ZH => "选择您喜欢的主题",
        Language::RO => "Selectați tema preferată",
        Language::KO => "태마를 선택하세요",
        Language::TR => "Favori temanızı seçin",
        Language::RU => "Выберите предпочительную тему",
        Language::PT => "Escolha o seu tema favorito",
        Language::EL => "Επίλεξε το αγαπημένο σου θέμα",
        // Language::FA => "زمینه دلخواه خود را انتخاب کنید",
        Language::SV => "Välj ditt favorittema",
        Language::FI => "Valitse suosikkiteemasi",
        Language::JA => "テーマを選択してください",
        Language::UZ => "Sevimli mavzuingizni tanlang",
        Language::VI => "Chọn chủ đề bạn muốn",
    })
}

pub fn active_filters_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Active filters",
        Language::IT => "Filtri attivi",
        Language::FR => "Filtres actifs",
        Language::ES => "Filtros activos",
        Language::PL => "Aktywne filtry",
        Language::DE => "Aktive Filter",
        Language::UK => "Активні фільтри",
        Language::ZH => "活动的过滤器",
        Language::RO => "Filtre active",
        Language::KO => "활성화된 필터",
        Language::TR => "Aktif filtreler",
        Language::RU => "Выбранные фильтры",
        Language::PT => "Filtros ativos",
        Language::EL => "Ενεργά φίλτρα",
        // Language::FA => "صافی های فعال",
        Language::SV => "Aktiva filter",
        Language::FI => "Aktiiviset suodattimet",
        Language::JA => "適用されているフィルター",
        Language::UZ => "Faol filtrlar",
        Language::VI => "Bộ lọc đang hoạt động",
    }
}

pub fn none_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "none",
        Language::IT => "nessuno",
        Language::FR => "aucun",
        Language::ES => "ninguno",
        Language::PL => "brak",
        Language::DE => "Keine",
        Language::UK => "бракує",
        Language::ZH => "无",
        Language::RO => "niciunul",
        Language::KO => "없음",
        Language::TR => "hiç biri",
        Language::RU => "ничего",
        Language::PT => "nenhum",
        Language::EL => "κανένα",
        // Language::FA => "هیچ کدام",
        Language::SV => "inga",
        Language::FI => "ei mitään",
        Language::JA => "なし",
        Language::UZ => "hech biri",
        Language::VI => "không có",
    }
}

pub fn yeti_night_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original dark theme",
        Language::IT => "Il tema scuro originale di Sniffnet",
        Language::FR => "Thème original sombre de Sniffnet",
        Language::ES => "Tema oscuro original de Sniffnet",
        Language::PL => "Oryginalny, ciemny motyw Sniffnet",
        Language::DE => "Sniffnets urspüngliches, dunkles Design",
        Language::UK => "Оригінальний, темний мотив Sniffnet",
        Language::ZH => "Sniffnet暗黑",
        Language::RO => "Tema întunecată originală Sniffnet",
        Language::KO => "Sniffnet의 기본 다크테마",
        Language::TR => "Sniffnet'in orjinal koyu teması",
        Language::RU => "Оригинальная тёмная тема Sniffnet'а",
        Language::PT => "Tema escuro original de Sniffnet",
        Language::EL => "Το αυθεντικό σκούρο θέμα του Sniffnet",
        // Language::FA => "زمینه تاریک اصلی Sniffnet",
        Language::SV => "Sniffnets ursprungliga mörka tema",
        Language::FI => "Sniffnetin alkuperäinen tumma teema",
        Language::JA => "Sniffnet のオリジナル テーマ",
        Language::UZ => "Sniffnet-ning asl qora mavzusi",
        Language::VI => "Chủ đề tối của Sniffnet",
    }
}

pub fn yeti_day_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sniffnet's original light theme",
        Language::IT => "Il tema chiaro originale di Sniffnet",
        Language::FR => "Thème original clair de Sniffnet",
        Language::ES | Language::PT => "Tema claro original de Sniffnet",
        Language::PL => "Oryginalny, jasny motyw Sniffnet",
        Language::DE => "Sniffnets urspüngliches, helles Design",
        Language::UK => "Оригінальний, світлий мотив Sniffnet",
        Language::ZH => "Sniffnet浅色",
        Language::RO => "Tema deschisă originală Sniffnet",
        Language::KO => "Sniffnet의 기본 라이트테마",
        Language::TR => "Sniffnet'in orjinal açık teması",
        Language::RU => "Оригинальная светая тема Sniffnet'а",
        Language::EL => "Το αυθεντικό ανοιχτόχρωμο θέμα του Sniffnet",
        // Language::FA => "زمینه روشن اصلی Sniffnet",
        Language::SV => "Sniffnets ursprungliga ljusa tema",
        Language::FI => "Sniffnetin alkuperäinen vaalea teema",
        Language::JA => "Sniffnet のオリジナル ライト テーマ",
        Language::UZ => "Sniffnet-ning asl oq mavzusi",
        Language::VI => "Chủ đề sáng của Sniffnet",
    }
}

pub fn deep_sea_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "To dive into network traffic",
        Language::IT => "Per immergersi nel traffico di rete",
        Language::FR => "Pour plonger dans votre trafic réseau",
        Language::ES => "Para sumergirse en el tráfico de la red",
        Language::PL => "Aby zanurzyć się w ruchu sieciowym",
        Language::DE => "Um in den Netzwerkverkehr einzutauchen",
        Language::UK => "Проаналізувати мережевий рух",
        Language::ZH => "潜入网络活动的海洋",
        Language::RO => "Pentru a vă scufunda în traficul de rețea",
        Language::KO => "네트워크 트레픽으로 바로가기",
        Language::TR => "Ağ trafiğine dalmak",
        Language::RU => "Для погружения в сетевой трафик",
        Language::PT => "Para mergulhar no tráfego de rede",
        Language::EL => "Βουτιά μέσα στην κίνηση του δικτύου",
        // Language::FA => "شیرجه رفتن در آمد و شد شبکه",
        Language::SV => "För att dyka ned i nätverkstrafiken",
        Language::FI => "Sukeltaaksesi verkkoliikenteeseen",
        Language::JA => "ネットワーク トラフィックにダイブ",
        Language::UZ => "Tarmoq trafigiga qo'shilish uchun",
        Language::VI => "Đắm chìm vào lưu lượng mạng",
    }
}

pub fn mon_amour_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Lovely theme made for dreamers",
        Language::IT => "Tema incantevole fatto per i sognatori",
        Language::FR => "Thème romantique fait pour les rêveurs",
        Language::ES => "Tema encantador hecho para soñadores",
        Language::PL => "Uroczy motyw stworzony dla marzycieli",
        Language::DE => "Schönes Design für Träumer",
        Language::UK => "Прекрасна тема для мрійників",
        Language::ZH => "梦想家的主题",
        Language::RO => "O temă minunată creată pentru visători",
        Language::KO => "사랑스러운 몽환가들을 위한 테마",
        Language::TR => "Hayal perestler için yapılmış güzel tema",
        Language::RU => "Милая тема для мечтателей",
        Language::PT => "Tema encantador feito para sonhadores",
        Language::EL => "Φτιαγμένο για ονειροπόλους",
        // Language::FA => "زمینه دلپذیر ساخته شده برای رویا پردازان",
        Language::SV => "Ljuvligt tema gjort för drömmare",
        Language::FI => "Ihana teema unelmoijille",
        Language::JA => "ドリーマーのためのテーマ",
        Language::UZ => "Xayolparastlar uchun chiroyli mavzu",
        Language::VI => "Chủ đề mộng mơ cho những kẻ mơ mộng",
    }
}

pub fn incoming_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Incoming",
        Language::IT => "In entrata",
        Language::FR => "Entrant",
        Language::ES => "Entrante",
        Language::PL => "Przychodzące",
        Language::DE => "Eingehend",
        Language::UK => "Вхідні",
        Language::ZH => "入站",
        Language::RO => "de intrare",
        Language::KO => "수신중",
        Language::TR => "Gelen",
        Language::RU => "Входящий",
        Language::PT => "Entrando",
        Language::EL => "Εισερχόμενα",
        // Language::FA => "ورودی",
        Language::SV => "Inkommande",
        Language::FI => "Saapuva",
        Language::JA => "受信",
        Language::UZ => "Kiruvchi",
        Language::VI => "Đang tới",
    }
}

pub fn outgoing_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Outgoing",
        Language::IT => "In uscita",
        Language::FR => "Sortant",
        Language::ES => "Saliente",
        Language::PL => "Wychodzące",
        Language::DE => "Ausgehend",
        Language::UK => "Вихідні",
        Language::ZH => "出站",
        Language::RO => "de ieșire",
        Language::KO => "발신중",
        Language::TR => "Giden",
        Language::RU => "Исходящий",
        Language::PT => "Saindo",
        Language::EL => "Εξερχόμενα",
        // Language::FA => "خروجی",
        Language::SV => "Utgående",
        Language::FI => "Lähtevä",
        Language::JA => "送信",
        Language::UZ => "Chiquvchi",
        Language::VI => "Đang hướng ra ngoài",
    }
}




pub fn language_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Language",
        Language::IT => "Lingua",
        Language::FR => "Langue",
        Language::ES => "Idioma",
        Language::PL => "Język",
        Language::DE => "Sprache",
        Language::UK => "Мова",
        Language::ZH => "语言",
        Language::RO => "Limbă",
        Language::KO => "언어",
        Language::TR => "Dil",
        Language::RU => "Язык",
        Language::PT => "Língua",
        Language::EL => "Γλώσσα",
        // Language::FA => "زبان",
        Language::SV => "Språk",
        Language::FI => "Kieli",
        Language::JA => "表示言語",
        Language::UZ => "Til",
        Language::VI => "Ngôn ngữ",
    }
}

pub fn overview_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Overview",
        Language::IT => "Panoramica",
        Language::FR => "Résumé",
        Language::ES => "Resumen",
        Language::PL => "Przegląd",
        Language::DE => "Übersicht",
        Language::UK => "Огляд",
        Language::ZH => "概览",
        Language::RO => "Prezentare generală",
        Language::KO => "개요",
        Language::TR => "Ön izleme",
        Language::RU => "Обзор",
        Language::PT => "Visão geral",
        Language::EL => "επισκόπηση",
        // Language::FA => "نمای کلی",
        Language::SV => "Översikt",
        Language::FI => "Yleiskatsaus",
        Language::JA => "概要",
        Language::UZ => "Ko'rib chiqish",
        Language::VI => "Tổng quan",
    }
}

pub fn packets_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a packets threshold is exceeded",
        Language::IT => "Notificami quando una soglia di pacchetti è superata",
        Language::FR => "Me notifier lorsqu'un seuil de paquet est atteint",
        Language::ES => "Notificarme cuando se supere un límite de paquetes",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg pakietów",
        Language::DE => "Benachrichtige mich, wenn die Pakete eine Schwelle überschreiten",
        Language::UK => "Повідом мене про переліміт пакетів",
        Language::ZH => "超过设定的数据包数量阈值时通知我",
        Language::RO => "Anunță-mă când este depășit un prag de pachete",
        Language::KO => "패킷 임계값을 초과하면 알림",
        Language::TR => "Paket eşiği aşıldığında beni bilgilendir",
        Language::RU => "Уведомить, когда порог по частоте пакетов превышен",
        Language::PT => "Notifique-me quando um limite de pacotes for excedido",
        Language::EL => "Ειδοποίησέ με όταν το όριο τον πακέτων ξεπεραστεί",
        // Language::FA => "به من اطلاع بده وقتی آستانه یک بسته فراتر رفت",
        Language::SV => "Notifiera mig när en paketgräns har överstigits",
        Language::FI => "Ilmoita minulle, kun pakettiraja on ylittynyt",
        Language::JA => "パケット数の閾値を超過した場合に通知する",
        Language::UZ => "Paket chegarasi oshib ketganda xabar bering",
        Language::VI => "Báo cho tôi biết khi vượt quá ngưỡng gói tin",
    }
}

pub fn bytes_threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when a bytes threshold is exceeded",
        Language::IT => "Notificami quando una soglia di byte è superata",
        Language::FR => "Me notifier lorsqu'un seuil de donnée est atteint",
        Language::ES => "Notificarme cuando se exceda un límite de bytes",
        Language::PL => "Powiadom mnie, gdy zostanie przekroczony próg bajtów",
        Language::DE => "Benachrichtige mich, wenn die Bytes eine Schwelle überschreiten",
        Language::UK => "Повідом мене про переліміт байтів",
        Language::ZH => "超过设定的网络流量阈值时通知我",
        Language::RO => "Anunță-mă când este depășit un prag de octeți",
        Language::KO => "바이트 임계값을 초과하면 알림",
        Language::TR => "Bayt eşiği aşıldığında beni bilgilendir",
        Language::RU => "Уведомить, когда порог в байтах превышен",
        Language::PT => "Notifique-me quando um limite de bytes for excedido",
        Language::EL => "Ειδοποίησέ με όταν το όριο των bytes ξεπεραστεί",
        // Language::FA => "به من اطلاع بده وقتی آستانه یک بایت فراتر رفت",
        Language::SV => "Notifiera mig när en gräns för bytes har överstigits",
        Language::FI => "Ilmoita minulle, kun tavuraja on ylittynyt",
        Language::JA => "バイト量の閾値を超過した場合に通知する",
        Language::UZ => "Bayt chegarasi oshib ketganda menga xabar bering",
        Language::VI => "Báo cho tôi biết khi vượt quá ngưỡng bytes",
    }
}

pub fn per_second_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "(per second)",
        Language::IT => "(al secondo)",
        Language::FR => "(par seconde)",
        Language::ES | Language::PT => "(por segundo)",
        Language::PL => "(na sekundę)",
        Language::DE => "(pro Sekunde)",
        Language::UK => "(на секунду)",
        Language::ZH | Language::JA => "(每秒) ",
        Language::RO => "(pe secundă)",
        Language::KO => "(초당)",
        Language::TR => "(her saniye)",
        Language::RU => "(в секунду)",
        Language::EL => "(ανά δευτερόλεπτο)",
        // Language::FA => "(در ثانیه)",
        Language::SV => "(per sekund)",
        Language::FI => "(sekunnissa)",
        Language::UZ => "(soniyasiga)",
        Language::VI => "(trên giây)",
    }
}

pub fn specify_multiples_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "you can also specify 'K', 'M' and 'G'",
        Language::IT => "puoi anche specificare 'K', 'M' e 'G'",
        Language::FR => "vous pouvez également spécifier 'K', 'M' et 'G'",
        Language::ES => "también puede especificar 'K', 'M' y 'G'",
        Language::PL => "możesz również określić 'K', 'M' i 'G'",
        Language::DE => "du kannst auch 'K', 'M' und 'G' verwenden",
        Language::UK => "можеш також вибрати 'K', 'M' i 'G'",
        Language::ZH => "您可指定 'K', 'M', 'G'",
        Language::RO => "puteți specifica 'K', 'M', 'G'",
        Language::KO => "지정 가능합니다 'K', 'M', 'G'",
        Language::TR => "şunları da kullanabilirsin 'K', 'M' ve 'G'",
        Language::RU => "Так же можно указать 'K', 'M' или 'G'",
        Language::PT => "também pode especificar 'K', 'M' e 'G'",
        Language::EL => "μπορείς επίσης να καθορίσεις τα 'K', 'M' και 'G'",
        // Language::FA => "؛ شما همچنین می توانید 'M'، 'K' و 'G' را تعیین کنید",
        Language::SV => "du kan också ange 'K', 'M' och 'G'",
        Language::FI => "voit myös määrittää 'K', 'M' tai 'G'",
        Language::JA => "'K', 'M', 'G' が選択可能です",
        Language::UZ => "'K', 'M' va 'G' ni ham belgilashingiz mumkin",
        Language::VI => "bạn cũng có thể chọn 'K', 'M' and 'G'",
    }
}

pub fn favorite_notification_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Notify me when new data are exchanged from my favorites",
        Language::IT => "Notificami quando nuovi dati sono scambiati dai miei preferiti",
        Language::FR => "Notifiez-moi lorsque des données sont échangées depuis mes favoris",
        Language::ES => "Notificarme cuando se intercambien nuevos datos de mis favoritos",
        Language::PL => "Powiadom mnie, gdy nowe dane z moich ulubionych zostaną wymienione",
        Language::DE => {
            "Benachrichtige mich, wenn neue Daten mit meinen Favoriten ausgetauscht werden"
        }
        Language::UK => "Повідом мене, коли буде обмін даними з моїх улюблених",
        Language::ZH => "收藏夹内的连接有新活动时通知我",
        Language::RO => "Anunță-mă când sunt transferate date noi de la favoritele mele",
        Language::KO => "즐겨찾기에서 새 데이터가 교환될 때 알림",
        Language::TR => "Favorilerimde veri akışı olduğunda beni uyar",
        Language::RU => "Уведомить, если произошёл обмен данными в соединениях из избранного",
        Language::PT => "Notificar-me quando novos dados forem trocados dos meus favoritos",
        Language::EL => "Ειδοποίησέ με όταν νέα δεδομένα έχουν ανταλλαγεί από τα αγαπημένα μου",
        // Language::FA => "به من اطلاع بده وقتی داده جدید از پسندیده های من مبادله شد",
        Language::SV => "Notifiera mig när ny data utbyts av mina favoriter",
        Language::FI => "Ilmoita minulle, kun suosikkini vaihtavat uusia tietoja",
        Language::JA => "お気に入りに指定したホストに関してデータ送受信があった場合に通知する",
        Language::UZ => "Sevimlilar ro'yhatidan yangi ma'lumotlar almashganda xabar bering",
        Language::VI => "Báo cho tôi biết khi dữ liệu mới được trao đổi từ mục yêu thích của tôi",
    }
}

pub fn threshold_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Threshold",
        Language::IT => "Soglia",
        Language::FR => "Seuil",
        Language::ES => "Límite",
        Language::PL => "Próg",
        Language::DE => "Schwellenwert",
        Language::UK => "Ліміт",
        Language::ZH => "阈值",
        Language::RO => "Prag",
        Language::KO => "임계값",
        Language::TR => "Eşik",
        Language::RU => "Порог",
        Language::PT => "Limite",
        Language::EL => "όριο",
        // Language::FA => "آستانه",
        Language::SV => "Gräns",
        Language::FI => "Raja",
        Language::JA => "閾値",
        Language::UZ => "Eshik",
        Language::VI => "Ngưỡng",
    }
}

pub fn volume_translation(language: Language) -> &'static str {
    match language {
        Language::EN | Language::IT | Language::FR | Language::PT => "Volume",
        Language::ES => "Volumen",
        Language::PL => "Głośność",
        Language::DE => "Lautstärke",
        Language::UK => "Гучність",
        Language::ZH | Language::JA => "通知音量",
        Language::RO => "Volum",
        Language::KO => "볼륨",
        Language::TR => "Ses",
        Language::RU => "Объём",
        Language::EL => "Ένταση",
        // Language::FA => "حجم",
        Language::SV => "Volym",
        Language::FI => "Äänenvoimakkuus",
        Language::UZ => "Ovoz balandligi",
        Language::VI => "Âm lượng",
    }
}

pub fn sound_translation(language: Language) -> &'static str {
    match language {
        Language::EN => "Sound",
        Language::IT => "Suono",
        Language::FR => "Son",
        Language::ES => "Sonido",
        Language::PL => "Dźwięk",
        Language::DE => "Ton",
        Language::UK | Language::RU => "Звук",
        Language::ZH | Language::JA => "通知音",
        Language::RO => "Sunet",
        Language::KO => "사운드",
        Language::TR => "Ses",
        Language::PT => "Som",
        Language::EL => "Ήχος",
        // Language::FA => "صدا",
        Language::SV => "Ljud",
        Language::FI => "Ääni",
        Language::UZ => "Ovoz",
        Language::VI => "Âm thanh",
    }
}

