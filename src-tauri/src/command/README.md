# this filie is intend to record all event in this application.

## Login

### devicecode_init
- Description: This event is triggered when the user clicks the login button.
- Parameters: None
- Return: 
  - Success:
    ```json
    {
        "user_code": "user_code",
        "verification_uri": "verification_uri",
        "expires_in": "expires_in"
    }
    ```  
    - Failure:
        ```json
        {
            "error": "error",
            "description": "error_description"
        }
        ```
  
