[
██████  ███████ ███████ ███████  █████  ██████   ██████  ████████
██   ██ ██         ███     ███  ██   ██ ██   ██ ██    ██    ██    
██████  █████     ███     ███   ███████ ██████  ██    ██    ██    
██   ██ ██       ███     ███    ██   ██ ██   ██ ██    ██    ██    
██████  ███████ ███████ ███████ ██   ██ ██████   ██████     ██
](https://t.me/Ym90X2JlX3ph_bot)
Bezzabot это telegram bot для всяких часто используемых команд для работы и не только.
Проект был создан в образовательных целях и работает на raspberry pi 2 model B.

Bot helper for frequently used things mostly related to software development.

## Доступные команды (Available commands): 

/help — Отображает help.

/skb — Превращает йцукен -> qwerty. Пример: `/skb <text> <layout> <from_lang> <to_lang>`. /skb йцук выведет qwer

/utime — Превращает unix timestamp в дату в формате %Y-%m-%d %H:%M:%S. Пример: `/utime 1` -> , 1970-01-01 00:00:01

## Кратко о технической части

Бот написан на Rust с применением библиотеки teloxide. Передача обновлений 
через webhook. Все это добро запущено на [raspberry pi 2 model b](https://amperka.ru/product/raspberry-pi-2-model-b),
которая работает у меня дома.

## Планы (Roadmap):
Команды: Base64, jwt, md5
Грузить layout из файлов через указание LAYOUT_DIR

