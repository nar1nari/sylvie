cat = котик
    .description = 😺 Показать картинку с котиком

cat-not-found = 😿 Котик не найден.

rule34 = rule34
    .description = 🔞 Показать случайное r34-изображение по тегам
    .tags = теги
    .tags-description = Теги, разделённые пробелом

safebooru = safebooru
    .description = 🖼️ Показать случайное аниме-изображение по тегам
    .tags = теги
    .tags-description = Теги, разделённые пробелом

coin = монетка
    .description = 🪙 Подбросить монетку
    .side = сторона
    .side-description = Выберите сторону

coin-msg = Вы выбрали `{$side}`... Выпала `{$correct}`!
coin-right = 🏅 Вы угадали!
coin-wrong = ⛔ Вы не угадали!
Heads = Орёл
Tails = Решка

ping = пинг
    .description = 😴 Проверить время отклика бота
ping-pong = Понг!

help = помощь
    .description = 🆘 Показать меню помощи
    .command = команда
    .command-description = Название команды

help-title = ℹ️ Помощь
help-usage = ℹ️ Использование:
help-optional =  (необязательно)
help-parameters = *️⃣ Параметры
help-not-found = ⛔ Команда не найдена
help-no-description = Описание отсутствует.
help-aliases = 🔣 Алиасы
help-not-found-description = Команда `{$command}` не существует.
help-footer = Команды можно вызывать через `/` или `s.`
    Используйте `/help [команда]` для подробной информации.

ask = спросить
    .description = 💬 Задать вопрос чат-боту
    .text = текст
    .text-description = Текст сообщения
ask-error = 🤖 Ответ не получен. Возможно, он был отфильтрован или возникла ошибка сети.

forget = забыть
    .description = 🗑️ Удалить записи из памяти
    .indexes = индексы
    .indices-description = Индексы записей из /memory

forget-forgot = 🚮 Записи с индексами {$indices} удалены из памяти.

forgetall = забытьвсё
    .description = 🗑️ Очистить историю
    .confirmation = подтверждение
    .confirmation-description = Напишите "yes", чтобы подтвердить.

forgetall-confirm = ⛔ Действие не подтверждено.
forgetall-forgot = 🚮 Вся история чата удалена.

restart = перезапуск
    .description = 🔄 Перезапустить чат-бота

restart-restarted = 🔄 Перезапуск завершён успешно.
restart-not = ⛔ Перезапуск не требуется.

memory = память
    .description = 💾 Просмотр постоянной памяти

memory-title = Записи в постоянной памяти:
memory-empty = Память пуста.
memory-footer = Используйте `s.forget ID`, чтобы удалить запись.
    Страница {$page} из {$max_page}

clear = очистить
    .description = 🗑️ Удалить последние сообщения в канале
    .amount = количество
    .amount-description = Количество сообщений для удаления

clear-last-msg-fail = ⛔ Не удалось получить последние сообщения.
clear-deleted = 🚮 Удалено {$amount} сообщений.

hug = обнять
    .description = 🤗 Обнять кого-то
    .target = цель
    .target-description = Участник, которого нужно обнять
hug-target = {$author} тепло обнимает {$target}
hug-no-target = Сильви тепло обнимает {$author}

kiss = поцеловать
    .description = 😘 Поцеловать кого-то
    .target = цель
    .target-description = Участник, которого нужно поцеловать
kiss-target = {$author} поцеловал {$target}
kiss-no = Извини, Сильви не интересуется такими вещами.

bite = кусь
    .description = 🧛 Куснуть кого-то
    .target = цель
    .target-description = Участник, которого нужно укусить
bite-target = {$author} кусает {$target}
bite-no-target = Сильви кусает {$author}

lick = лизь
    .description = 👅 Лизнуть кого-то
    .target = цель
    .target-description = Участник, которого нужно лизнуть
lick-target = {$author} лижет {$target}
lick-no-target = Сильви лижет {$author}

pat = гладь
    .description = 🫳 Погладить по голове
    .target = цель
    .target-description = Участник, которого нужно погладить
pat-target = {$author} мягко гладит {$target}
pat-no-target = Сильви мягко гладит {$author}

sleep = спать
    .description = 😴 Заснуть
    .target = цель
    .target-description = Участник, с которым нужно лечь спать
sleep-target = {$author} спит с {$target}
sleep-no-target = {$author} спит

rp-need-target = 👤 Для этого действия нужно указать цель.
booru-not-found = 🗨️ Изображения с такими тегами не найдены.
went-wrong = Что-то пошло не так, попробуйте ещё раз.
error = Ошибка: {$error}
argument-parse-error = **Неверные аргументы команды.**
    Проверьте правильность использования в меню помощи.
nsfw-error = **Эта команда доступна только в NSFW-каналах.**
user-permissions-error = **Недостаточно прав:** `{$permissions}`
bot-permissions-error = "**Боту не хватает прав:** `{$permissions}`
guild-only-error = **Команда доступна только на сервере.**
error-no-reference = **Исходное сообщение неизвестно.**
