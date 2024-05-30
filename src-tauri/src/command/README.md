# this filie is intend to record all command in this application.

## Login

### devicecode_init
- Description: This command is triggered when the user clicks the login button.
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
          "status": "error",
          "description": "error_description"
      }
      ```
### devicecode_exchange
- Description: This event is triggered when frontend receives the user_code after the devicecode_init command.
- Parameters: None
- Return: 
  - Success:
    ```json
    {
        "status": "success",
        "description": "success_description" 
    }
      ```
    or
    ```json
    {
        "status": "success"
    }
    ```
  - Failure:
      ```json
      {
          "status": "error",
          "description": "error_description"
      }
      ```
### xbox_live_auth
- Description: This event is triggered after user enter device code and allow us to access xbox with token.
- Parameters: None
- Return:
  - Success:
    ```json
    {
        "status": "success",
        "description": "success_description" 
    }
      ```
    or
    ```json
    {
        "status": "success"
    }
    ```
  - Failure:
      ```json
      {
          "status": "error",
          "description": "error_description"
      }
      ```

### xbox_xsts_auth
- Description: This event is triggered after xbox_live_auth command.
- Parameters: None
- Return:
  - Success:
    ```json
    {
        "status": "success",
        "description": "success_description" 
    }
      ```
    or
    ```json
    {
        "status": "success"
    }
    ```
  - Failure:
      ```json
      {
          "status": "error",
          "description": "error_description"
      }
      ```
    
### minecraft_token
- Description: This event is triggered after xbox_xsts_auth command, to get minecraft token.
- Parameters: None
- Return:
  - Success:
    ```json
    {
        "status": "success",
        "description": "success_description" 
    }
      ```
    or
    ```json
    {
        "status": "success"
    }
    ```
  - Failure:
      ```json
      {
          "status": "error",
          "description": "error_description"
      }
      ```
    
### minecraft_profile
- Description: This event is triggered after minecraft_token command, to get minecraft profile and check if player has game or not.
- Parameters: None
- Return:
  - Success:
    ```json
    {
        "status": "success",
        "description": "success_description" 
    }
      ```
    or
    ```json
    {
        "status": "success"
    }
    ```
  - Failure:
      ```json
      {
          "status": "error",
          "description": "error_description"
      }
      ```
