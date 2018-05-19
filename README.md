# vrc-invite-bot

VRChat bot to automate sending invites

## Usage

The bot consists of 2 commands, one to accept all incoming friend requests (since only friends can send invite) and one to handle invite requests

### Friend requests

```bash
vr-invite-bot <api_key> <username> <password> accept
``` 

### Invite requests

```bash
vr-invite-bot <api_key> <username> <password> invite
``` 

### Requesting an invite

You can request an invite into a world from the bot by sending an "requestinvite" notification to the bot with the following details:

```typescript
{
  world: string,
  instance: string
}
```

any message in the notification will be forwarded to the resulting invite.