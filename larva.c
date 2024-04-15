#include "libavcodec/codec.h"
#include <stdio.h>

int main() {
  char *larbario = "libsvtav1";
  const AVCodec *mosca = avcodec_find_encoder_by_name(larbario);

  if (mosca == NULL) {
    printf("Que te jodan\n");
    return 1;
  } else {
    printf("pulga %s\n", mosca->long_name);
  }
}