<a href="https://t.me/Ym90X2JlX3ph_bot">
    <pre>
    ██████  ███████ ███████ ███████  █████  ██████   ██████  ████████
    ██   ██ ██         ███     ███  ██   ██ ██   ██ ██    ██    ██    
    ██████  █████     ███     ███   ███████ ██████  ██    ██    ██    
    ██   ██ ██       ███     ███    ██   ██ ██   ██ ██    ██    ██    
    ██████  ███████ ███████ ███████ ██   ██ ██████   ██████     ██
    </pre>
</a>

Bezzabot это telegram bot для разработчика. Сборник часто используемых команд, за которыми, обычно,
обращаешься к специальным сервисам. Проект создан в образовательных целях и работает на raspberry pi 2 model B.

Bot helper for frequently used things mostly related to software development.

## Локальная разработка (Local development)
1. Установить Rust ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh```
2. Склонировать репозиторий ```git clone git@github.com:radiopapus/bezzabot.git```
3. Создать бота в телеграм через https://t.me/botfather
От бота нужен токен, дальше TOKEN_FROM_TELEGRAM.
4. Установить и запустить ngrok ```sudo apt install ngrok ```, ```ngrok http 8081```
Скопировать адрес вида https://какая-то-строка.ngrok-free.app в буфер обмена, дальше NGROK_HOST_URL
5. Заполнить переменные окружения
HOST_URL=NGROK_HOST_URL
PORT=8081
TELOXIDE_TOKEN=TOKEN_FROM_TELEGRAM
6. Задать webhook url для телеграм, отправив POST запрос на
curl -X POST https://api.telegram.org/bot{TOKEN_FROM_TELEGRAM}/setWebhook?url={NGROK_HOST_URL}
7. Запустить bezzabot локально и в телеграм боте выполнить команду /help.

Если все ок, то бот отобразит список доступных команд.

Имейте в виду, что при перезапуске ngrok в бесплатной версии адрес будет изменяться и необходимо 
соответственно обновлять webhook url. 

## Команды (Available commands): 

```/help``` — Отображает текст с описанием команд.

```/skb``` — Превращает йцукен -> qwerty. Пример: /skb <text>.

```/utime``` — Превращает unix timestamp в дату в формате %Y-%m-%d %H:%M:%S. Пример: /utime ts -> , где ts число секунд с 01 января 1970.

```/qr``` — Генерирует qr code по тексту. Пример: /qr text

```/winner``` — Выбирает случайный id из списка. Пример: /winner 1 2 3 4 5

```/encode -f {b64, url} text``` — Кодирует строку в заданном формате. base84, urlencode. 

```/decode -f {b64, url} text``` — Декодирует строку в заданном формате. base84, urlencode.

```/jp``` — Json pretty print. /jp json_string

```/radix``` — Конвертер из одной системы счисления в другую. /radix 2 16 -> 1111 /radix 16 2 af -> 0b10101111

## Кратко о технической части

Бот написан на Rust с применением библиотеки teloxide. Передача обновлений TG через webhook. 
Работает на [raspberry pi 2 model b](https://amperka.ru/product/raspberry-pi-2-model-b).
Вкратце тут : [Telegram бот на Rust, Два, Три и Raspberry Pi 2](https://habr.com/ru/articles/720410/), чуть подробнее 
на [видео](https://www.youtube.com/watch?v=ubCFcG0HtOo)
