#include <stdio.h>
#include <stdlib.h>
#include <sys/types.h>
#include <sys/ipc.h>
#include <sys/msg.h>
#include <string.h>
#include "polymsgtypes.h"

int main(int argc, char * argv[])
{
  struct msg_1_t message1;
  struct msg_2_t message2;
  int msgid;
  int wait_mode=1;
  char * response = "Ok!";
  msgid = msgget(KEY, 0777 | IPC_CREAT);  // создаем очередь сообщений
  while (1==1)
  {
    msgrcv(msgid, &message1, MSG_1_SIZE, 1, 0);  // ждем сообщение
    printf("Client (pid = %i) sent: %s", message1.snd_pid, message1.body);
    message2.mtype = 2;
    message2.snd_pid = getpid();
    message2.rcv_pid = message1.snd_pid;
    // if (message1.body == "wait")
    // if (message1.client_wait_req == "1")
    if (1==1)
    
    {
      wait_mode = 1;
      strcpy(message2.body, response);
      msgsnd(msgid, &message2, MSG_2_SIZE, 0); // посылаем ответ
      msgrcv(msgid, &message1, MSG_1_SIZE, 1, 0);  // ждем подтверждения

      printf ("'%i' not found\n", message1.client_wait_req);
    } else {
      wait_mode = 0;
      strcpy(message2.body, response);
      msgsnd(msgid, &message2, MSG_2_SIZE, 0); // посылаем ответ
      msgrcv(msgid, &message1, MSG_1_SIZE, 1, 0);  // ждем подтверждения
    }
    

  }
  msgctl(msgid, IPC_RMID, 0);  // удаляем очередь
  
  return EXIT_SUCCESS;
}
