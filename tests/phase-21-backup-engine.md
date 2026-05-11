# Тесты: Этап 21 — Backup Engine

> 3-2-1 strategy, USB, S3, P2P, incremental, encryption, recovery phrase, scheduled, corruption detection, parallel streams. Все тесты на реальных данных и реальных носителях.

---

### TC-21-001: Full backup — USB
**Тип:** E2E | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Вставить USB. 2. "backup to USB".
**Ожидаемый результат:**
- Backup < 5 мин (1 GB). Incremental: только изменения.
**Автоматизация:** ручной (USB).

### TC-21-002: Full backup — S3
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Configure S3. 2. "backup to S3".
**Ожидаемый результат:**
- Uploaded. Encryption: AES-256-GCM.
**Автоматизация:** автоматический (test bucket).

### TC-21-003: Full backup — P2P node
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Peer B как backup target. 2. Backup.
**Ожидаемый результат:**
- Передано. Encrypted.
**Автоматизация:** автоматический.

### TC-21-004: Incremental — только изменения
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Full backup. 2. Изменить 1 файл. 3. Incremental.
**Ожидаемый результат:**
- Только 1 файл. Time < 10 сек.
**Автоматизация:** автоматический.

### TC-21-005: Incremental — dedup
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Move файл (не изменить). 2. Incremental.
**Ожидаемый результат:**
- Passport update only. Blob не передаётся.
**Автоматизация:** автоматический.

### TC-21-006: 3-2-1 rule
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. USB + S3 + P2P.
**Ожидаемый результат:**
- 3 копии. 2 носителя. 1 offsite.
**Автоматизация:** автоматический.

### TC-21-007: Encryption — AES-256-GCM
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Backup encrypted. 2. Проверить: ciphertext.
**Ожидаемый результат:**
- Нет plaintext. Tag корректен.
**Автоматизация:** автоматический.

### TC-21-008: Recovery phrase — restore
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Backup encrypted. 2. Удалить всё. 3. Recovery phrase. 4. Restore.
**Ожидаемый результат:**
- Данные восстановлены.
**Автоматизация:** автоматический.

### TC-21-009: Recovery phrase — wrong phrase
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Неверная phrase.
**Ожидаемый результат:**
- `DecryptionError`. Нет partial restore.
**Автоматизация:** автоматический.

### TC-21-010: Scheduled — daily
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Daily at 3 AM. 2. Mock time.
**Ожидаемый результат:**
- Backup запущен в 3 AM ± 1 мин.
**Автоматизация:** автоматический.

### TC-21-011: Scheduled — weekly
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Weekly, Sunday.
**Ожидаемый результат:**
- Запущен в воскресенье.
**Автоматизация:** автоматический.

### TC-21-012: Corruption detection — 1 bit flip
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Backup. 2. Flip 1 bit. 3. Restore.
**Ожидаемый результат:**
- `BackupCorrupted`. Restore отменён.
**Автоматизация:** автоматический.

### TC-21-013: Corruption detection — truncated file
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Truncate backup file.
**Ожидаемый результат:**
- `BackupCorrupted`.
**Автоматизация:** автоматический.

### TC-21-014: Parallel streams — 4 streams
**Тип:** Performance | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Backup 1000 файлов. 2. Sequential vs parallel.
**Ожидаемый результат:**
- Parallel > 2x faster.
**Автоматизация:** автоматический.

### TC-21-015: Parallel streams — adaptive
**Тип:** Performance | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Slow network. 2. Adaptive streams.
**Ожидаемый результат:**
- Fewer streams. Стабильно.
**Автоматизация:** автоматический.

### TC-21-016: Compression — ratio
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Backup text files.
**Ожидаемый результат:**
- Ratio > 2:1.
**Автоматизация:** автоматический.

### TC-21-017: Compression — already compressed
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Backup PNG/JPEG.
**Ожидаемый результат:**
- Store (no compression overhead).
**Автоматизация:** автоматический.

### TC-21-018: Resume — interrupted backup
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P0
**Шаги:**
1. Backup 50%. 2. Interrupt. 3. Resume.
**Ожидаемый результат:**
- Resume с 50%. Нет повторной передачи.
**Автоматизация:** автоматический.

### TC-21-019: Versioning — backup history
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. 5 backups. 2. List versions.
**Ожидаемый результат:**
- 5 versions. Restore any.
**Автоматизация:** автоматический.

### TC-21-020: Versioning — delete old
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Retention = 3. 2. 5 backups. 3. Prune.
**Ожидаемый результат:**
- 2 oldest deleted.
**Автоматизация:** автоматический.

### TC-21-021: Selective backup — by tag
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Backup only "work" tag.
**Ожидаемый результат:**
- Только tagged projects.
**Автоматизация:** автоматический.

### TC-21-022: Selective restore — by project
**Тип:** Integration | **Платформа:** All | **Данные:** Реальные | **Приоритет:** P1
**Шаги:**
1. Restore single project.
**Ожидаемый результат:**
- Только этот project. Остальные не затронуты.
**Автоматизация:** автоматический.
