#!/usr/bin/python3

NB_FIG = 6
TIME = 5

for i in range(NB_FIG):
    print("""figure:nth-child({}) {{
    animation: xfade {}s {}s infinite;
}}""".format(i+1, TIME * NB_FIG, TIME * (NB_FIG - i - 1)))

print("""

@keyframes xfade {{
  0% {{
    opacity: 1;
  }}
  {}% {{
    opacity: 1;
  }}
  {}% {{
    opacity: 0;
  }}
  98% {{
    opacity: 0;
  }}
  100% {{
    opacity: 1;
  }}
}}
""".format(100/NB_FIG, 100/NB_FIG*1.5))
