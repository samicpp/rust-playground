#include<stdio.h>
#include<string.h>

int main(int argc,char *argv[]){
    int tot=0;
    for(int i=1;i<argc;i++){
        tot+=strlen(argv[i])+1;
    };
    char str[tot];
    str[0]='\0';
    for(int i=1;i<argc;i++){
        strcat(str,argv[i]);
        if(i<argc-1){
            strcat(str," ");
        }
    };
    printf("%s\r\n",str);
    return 0;
}