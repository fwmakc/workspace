# Этап 25 — Email & VoIP

## Цель
Добавить поддержку электронной почты (IMAP/SMTP) и голосовых/видео звонков (WebRTC через WireGuard). После этого этапа пользователь может отправлять письма и звонить другим пользователям CORE.

## Язык и стек
- **Язык:** TypeScript
- **Runtime:** Bun
- **Ключевые зависимости:** `emailjs-imap-client` или кастомный IMAP/SMTP client, `werift-webrtc` или `node-datachannel` (WebRTC data channels), `bun:udp` (SRTP через WireGuard)
- **Целевые ОС:** Windows, macOS, Linux, Android

## Зависимости
- **Этап 8** — Host Shim: Network (TCP/UDP, WireGuard).
- **Этап 19** — P2P Mesh (WireGuard туннели, peer discovery).
- **Этап 24** — Messenger Engine (контакты, уведомления, UI шаблоны).

## Часть системы
**Level 1 — Бэк: Email + VoIP** [См. layer-8 §6.2–6.3, layer-1 §5, layer-5 §5]

## Требования

### 23.1 Email (IMAP/SMTP)
- **Accounts:** SQLite таблица `email_accounts` (address, imap_server, smtp_server, credentials encrypted).
- **IMAP:**
  - Подключение к серверу (STARTTLS или SSL/TLS).
  - Синхронизация папок: Inbox, Sent, Drafts, Trash.
  - Fetch headers + body (lazy: body загружается по требованию).
  - IDLE mode: поддержка push-уведомлений от сервера (RFC 2177).
  - Full-text search: FTS5 по заголовкам и телам писем (кэшируется локально).
- **SMTP:**
  - Отправка писем с поддержкой MIME (multipart, attachments).
  - Queue: если отправка не удалась — письмо ставится в очередь, повтор через 5 минут.
  - Drafts: автосохранение черновиков каждые 30 сек.
- **Security:**
  - Credentials хранятся в Key Manager (этап 26), зашифрованные.
  - OAuth2: placeholder (Gmail, Outlook) — базовый IMAP/SMTP на этом этапе.
- **UI (core.email):**
  - Список писем (sender, subject, snippet, date, unread).
  - Чтение письма (HTML rendering через Island Mode, этап 21).
  - Написание письма (композер с markdown + attachments).
  - Адресная книга интеграция (core.contactbook).

### 23.2 VoIP (CORE → CORE)
- **Сигналинг:** через P2P Mesh (этап 17). При звонке инициатор отправляет `call_offer` (SDP) через Messenger (этап 22).
- **Transport:** WebRTC data channels поверх WireGuard (не через публичные STUN/TURN, а через private P2P).
- **Media:**
  - Аудио: Opus, 48 kHz, stereo.
  - Видео: placeholder (без видео на этом этапе, только голосовые звонки). Видео — post-release.
- **ICE:** candidates через P2P (host candidates + reflexive через STUN если нужно).
- **Call state:** `idle` → `dialing` → `ringing` → `connected` → `ended` (или `missed`, `declined`).
- **Quality:** adaptive bitrate, jitter buffer (200 мс), packet loss concealment.
- **UI (core.voip):**
  - Incoming call overlay (поверх всех окон, через Display Server Overlay Layer).
  - In-call UI: mute, speaker, end call, duration.
  - Call history: список звонков с duration и outcome.

### 23.3 Contacts Integration
- Email-адреса и phone numbers хранятся в `core.contactbook` (этап 22).
- При написании письма или звонке — автодополнение из контактов.
- **Avatar sync:** аватары контактов синхронизируются через P2P (если контакт — пользователь CORE).

## Ключевые функции

| Функция | Описание | Тест |
|---------|----------|------|
| IMAP sync | Синхронизация | Подключиться → письма загружены |
| Send email | SMTP | Отправить письмо → в Sent, доставлено |
| Search email | FTS | Поиск "invoice" → находит письмо |
| VoIP call | Звонок | Позвонить → соединение, голос слышен |
| Call end | Завершение | Положить трубку → call history обновлён |
| Mute | Без звука | Нажать mute → собеседник не слышит |

## Интеграция с будущими этапами
- **Вход:** этап 6 (Network) — TCP/UDP, WireGuard.
- **Вход:** этап 17 (P2P) — signaling transport.
- **Вход:** этап 22 (Messenger) — contacts, message UI patterns.
- **Выход:** email/voip events → этап 15 (Window Manager, Overlay Layer для incoming call).
- **Выход:** voip audio → этап 4 (Host Shim Audio) для capture/playback.

## Критерии приёмки
- [ ] IMAP: синхронизация 100 писем < 30 сек.
- [ ] SMTP: отправка письма, получение на другом аккаунте < 1 мин.
- [ ] FTS: поиск по 1000 письмам < 100 мс.
- [ ] VoIP: соединение между двумя устройствами < 3 сек, latency < 200 мс (LAN).
- [ ] VoIP качество: MOS > 3.5 (при отсутствии packet loss).
- [ ] Call history: все звонки записаны с duration.
- [ ] Integration: email composer использует контакты из contactbook.

## Ссылки
- [layer-1-user-experience.md](../layers/layer-1-user-experience.md) — Messenger, контакты
- [layer-5-devices.md](../layers/layer-5-devices.md) — VoIP, медиа
- [layer-8-technical-decomposition.md](../layers/layer-8-technical-decomposition.md) — Email §6.2, VoIP §6.3
