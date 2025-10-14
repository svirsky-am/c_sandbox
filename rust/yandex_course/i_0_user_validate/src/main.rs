#[derive(Debug, PartialEq)]
enum ValidationError {
    EmailTooShort,
    EmailMissingAt,
    PasswordTooShort,
    AgeTooYoung,
}

#[derive(Debug, PartialEq)]
struct User {
    email: String,
    password: String,
    age: u8,
}

fn validate_email(email: &str) -> Result<String, ValidationError> {
    if email.len() < 5 {
        return Err(ValidationError::EmailTooShort);
    }

    if !email.contains('@') {
        return Err(ValidationError::EmailMissingAt);
    }

    Ok(email.to_string())
}

fn validate_password(password: &str) -> Result<String, ValidationError> {
    if password.len() < 8 {
        return Err(ValidationError::PasswordTooShort);
    }

    Ok(password.to_string())
}

fn validate_age(age: u8) -> Result<u8, ValidationError> {
    if age < 18 {
        return Err(ValidationError::AgeTooYoung);
    }

    Ok(age)
}

fn create_user(email: &str, password: &str, age: u8) -> Result<User, ValidationError> {
    let validated_email = validate_email(email)?;
    let validated_password = validate_password(password)?;
    let validated_age = validate_age(age)?;

    Ok(User {
        email: validated_email,
        password: validated_password,
        age: validated_age,
    })
}

fn main() {
    // Вызывающий код обрабатывает ошибки по-разному:

    // 1. Успешное создание пользователя
    match create_user("user@test.com", "password123", 25) {
        Ok(user) => println!("Пользователь создан: {:?}", user),
        Err(e) => println!("Ошибка: {:?}", e),
    }

    // 2. Обработка конкретных ошибок
    match create_user("abc", "123", 16) {
        Ok(user) => println!("Пользователь создан: {:?}", user),
        Err(ValidationError::EmailTooShort) => println!("Email слишком короткий!"),
        Err(ValidationError::EmailMissingAt) => println!("В email отсутствует @"),
        Err(ValidationError::PasswordTooShort) => println!("Пароль слишком короткий!"),
        Err(ValidationError::AgeTooYoung) => println!("Пользователь слишком молод!"),
    }

    // 3. Игнорирование ошибок (не рекомендуется, но возможно)
    if let Ok(user) = create_user("admin@site.com", "securepass", 30) {
        println!("Администратор: {:?}", user);
    }

    // 4. Использование unwrap_or для значения по умолчанию
    let default_age = validate_age(15).unwrap_or(18);
    println!("Возраст: {}", default_age);

    // Тесты
    assert_eq!(validate_email("ab"), Err(ValidationError::EmailTooShort));
    assert_eq!(
        validate_email("test.com"),
        Err(ValidationError::EmailMissingAt)
    );
    assert_eq!(
        validate_email("user@test.com"),
        Ok("user@test.com".to_string())
    );

    assert_eq!(
        validate_password("1234567"),
        Err(ValidationError::PasswordTooShort)
    );
    assert_eq!(
        validate_password("password123"),
        Ok("password123".to_string())
    );

    assert_eq!(validate_age(17), Err(ValidationError::AgeTooYoung));
    assert_eq!(validate_age(25), Ok(25));

    println!("Все тесты прошли! Поздравляем!");
}
