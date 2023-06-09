
#include<iostream>

void solution(char *p) {
  char the_key = 'A';
  std::cout<<"The value of the key is:"<<the_key<<std::endl;
  p = &the_key;
  std::cout<<"The address of the key is:"<<p<<std::endl;
}

int main() {
  char *p = NULL;

  solution(p);

  std::cout<<*p<<std::endl;

  return 0;
}

