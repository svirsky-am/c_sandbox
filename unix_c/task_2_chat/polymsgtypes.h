#ifndef MSG_TYPES
#define MSG_TYPES

#define KEY 1274     // "магическое" число 
#define MAXLEN 512

struct msg_1_t
{
   long mtype;
   int snd_pid;
   char body[MAXLEN];
   int client_wait_req;
   int client_req_end_session;
};

struct msg_2_t
{
   long mtype;
   int snd_pid;
   int rcv_pid;
   char body[MAXLEN];
   int client_wait_req;
   int client_req_end_session;
};

#define MSG_1_SIZE sizeof(struct msg_1_t) - sizeof(long)
#define MSG_2_SIZE sizeof(struct msg_2_t) - sizeof(long)

#endif
