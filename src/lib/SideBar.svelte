<script lang="ts">
  import Avatar from './Avatar.svelte'
  import PlusButton from './PlusButton.svelte'
  import figure from '../stores/Figure'
  import MainAvatar from './MainAvatar.svelte'

  export let onchange: (a_count: number) => void

  const change = (count: number) => {
    if (a_count !== count) {
      a_count = count
      onchange(count)
    }
  }

  console.log($figure)
  let a_count
  change(!!$figure[0] ? 0 : -1)
</script>

<div
  class={`main ${a_count === -1 ? '' : 'ov'} ${import.meta.env.TAURI_PLATFORM}`}
  data-tauri-drag-region>
  {#if import.meta.env.TAURI_PLATFORM === 'macos'}
    <div style="padding-top: 2em"></div>
  {/if}
  <div class={`box main-box ${a_count === -1 ? 'ac' : ''}`}>
    <MainAvatar
      onclick={() => {
        change(-1)
      }}
      iName="{'俗手'},"
      base64="/9j/4AAQSkZJRgABAgAAAQABAAD/wAARCADIAMgDAREAAhEBAxEB/9sAQwAGBAUGBQQGBgUGBwcGCAoQCgoJCQoUDg8MEBcUGBgXFBYWGh0lHxobIxwWFiAsICMmJykqKRkfLTAtKDAlKCko/9sAQwEHBwcKCAoTCgoTKBoWGigoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgoKCgo/8QAHwAAAQUBAQEBAQEAAAAAAAAAAAECAwQFBgcICQoL/8QAtRAAAgEDAwIEAwUFBAQAAAF9AQIDAAQRBRIhMUEGE1FhByJxFDKBkaEII0KxwRVS0fAkM2JyggkKFhcYGRolJicoKSo0NTY3ODk6Q0RFRkdISUpTVFVWV1hZWmNkZWZnaGlqc3R1dnd4eXqDhIWGh4iJipKTlJWWl5iZmqKjpKWmp6ipqrKztLW2t7i5usLDxMXGx8jJytLT1NXW19jZ2uHi4+Tl5ufo6erx8vP09fb3+Pn6/8QAHwEAAwEBAQEBAQEBAQAAAAAAAAECAwQFBgcICQoL/8QAtREAAgECBAQDBAcFBAQAAQJ3AAECAxEEBSExBhJBUQdhcRMiMoEIFEKRobHBCSMzUvAVYnLRChYkNOEl8RcYGRomJygpKjU2Nzg5OkNERUZHSElKU1RVVldYWVpjZGVmZ2hpanN0dXZ3eHl6goOEhYaHiImKkpOUlZaXmJmaoqOkpaanqKmqsrO0tba3uLm6wsPExcbHyMnK0tPU1dbX2Nna4uPk5ebn6Onq8vP09fb3+Pn6/9oADAMBAAIRAxEAPwDznbWFzAaVoAaUouMaUoHYjZadykRMlFyiJkouBGyCmO4wx0XKImSgBjJ7U7jIjHRcNRjJ7UDI2jpjGbKAGNHQIjZKBjClADCtAWGlaYrEZSgBpWgQwimMawpiaGEUhHq5SsDmuMK0DGFaBjGWgaIyKZQwrSGRsvWmMiZaAsMZeppFGdc6laQNtaXc/wDdQbj+lUkx2Kr61ar95ZlHqYzVWYh8Wp2MoJW4QY/vfL/Ok0xkkVzb3BIhlRz6A8/lQUPKUCIylFwsNZKdwI2jouMYyUDGFKAI2SmIjZKAI2SmBGVoEMIphsRkUCPXCtcxxDStAyNlplJkTCgZE60XKIyKZZEwoGNxk0FGBdGXVb5reFylpH99h3qlorl/CaVvaQWqbYIlQd8Dk/U0m2ybkjLu4AznjFTsM5XXo4Y7poYURVUneyD7x/wq4O6uW1YpXthLZQxzsr7Tg7lbBU+4xVrUhm/pl0s+nQyu2W24bucjioa1GtUOe8twfmZx9Y2/woHYiN7bf3yPqjD+lCEOW6t3OBNHn/exTsA8r3FIBjLTuAwrTuMjZKBETJTGQstAiJlpgRsKBbHrhrA4hjCgdiM0DsRt9KBkbCgpELigpELUFFPVpDBpskgOMsEz6ZBP8gaLXZpBdSLSofIsYwRh3+dvqf8A62BTb1E9WWSDmlcQwSpFchWYeYqtJtzyMAkZ/EVMndGsIO92chfymFlkAUuWz83Tjn+eK1RUmZ11qNzOWEzgoQRhVAHSqSSIbbKW5igVmYqOAM8Cq6kpaBt96LjsGz3xRcLDgjDoeKQ7EsFxPB/qpGX2zxQOxp22tkfLdR5H95f8KVhPQ1YJ4blN0EgYd8dR+FLYW45lxQBGy0xkTrTAgdaAIHGAaYj15krmucFyJlIplIiYUhkbUykRsKCiJhTHYiZaBmb4njL2um23/PWR8+6nYP6NTi9TVfCTySwxJ5s8yRQ5xvY9foOp/CpYKLZh6l4rSFGi0u3yW48+UfMfoB0pqDe5orR2MvRbqee+f7TOSGjYAdFBOOw4q5JW0HG7epHryYnjUEEBTyO/JpxBozWjCxcjLt09hTFYETfCSQNyttOP0oYkrkbIV6UBawoXI5FMLChPTigY4x8UgsRvFTCxGhkgkDxMVYdxTv3IceqNzTdWWfEVzhJegPZv8Klx7CUujNMrU3KsRstUmMgdaAKlxwKYWPYWFcp5pGwpjIXXigsgcGncoiNMoY9IpETUxmb42Uxyacts6yzQ7o5Nq5CudxK+5Hepg73Z0KNlZnH3EjzyPLdO8gT5Tk9T2Uen9K19B+pWS3eR8kfMeAB0HsKd7Ctct26iGdRH93kM397j+VIaJL2MSvGT0xg/ShDM+Vd0men9KoCW3iBjn+gNK4EbxfKeOlCYiLZnincQIp3bW+9296YyUJilcBpGDgjr0oAjkiFO4blSWPHammQ43NHS9WaIrDdnKdA/dfr7UON9USnZ2ZvNggEHIPIIqCyFxTGUZxk0x2PYmFcp5ZEwplETA0FIiYUFELrmmMgcEUFodBPHYxy3033YR8g/vSH7oH6n8KmevumkFdnL3by3dlOzjcxcSpGpILA/KT7LyB7/AK1asmdBkXKb7oQDBjtht4HBbufz4+gFUtrifYese1CR1bgfTv8A5+tACrFgdKYysTyY2PIOKYivszK49KLiLcEeDKvsKQ0IYcc0BYrvDjpTE0Mmi3x5HDDmmh2EhPmJg/eFMAdMgikBCp5waYhsqZBoAz5lwxq0zOSNPQ9QMbC2mPyMfkJ7H0+lKS6kwlbRm5NwDUG1ijIOaC0j2N1rnPHIXWgpELCgoiYUFIiccUyiJhQWZ3ikQ2rRfaV3R20QIiOcSTOAxJ/2VQx/jx60oXepvBWRz2hyOb9767LmNIHlZVOCRwox+LAj6VpK1rI0V92RJaNCgydwk+dXH8YPf/PTpRcCdovmx2XgUIdhpjouBl6ghEvmIOVHzVaAbbfPltpxkA4pAWoSGupdpyCBSGTlKdwIpIxnGOtAEDJgmmBRI8q5x2amSTuuOe1BVircJg5HehCBRvjJzyOtHURSnXP1FUiWim3Bq0YyR1FlcfabGNz94Da31FZyVmbQd0DLUmyR7M61znikDCgaInWmUQMtBSImWg0QkSJlpJnEcESmSRz0VQMk0FrU5jXzNrlzHd3BVUuJGKRr/DEvYfjgZ9TTjaCsuh0pXGXcHlaPPgYe4mSJPouSw/VfypRd2XIktIY3TazEeWAEXt9abYkiOYAucDgcD6U0URMlAGbegRSzZ6bAf61SEyjapLbsJlG+IH51HUe4q99BaxZZCxT3rMPmUqCCDip1RW7LoQKMDOPc5oHYZKvy5x05oE0QSp3pgZt+uFVx1U00QyzFiW3B9sUFIrzJlCO9MCrEfvD1GKGQV5eDVAypMuG9qpGcka2gNmGZPRgfz/8A1VMyqOt0aTCszpR7KwrA8IiccUDRC60FoiYUFIjEZdtqjJPQUFIzfFUPm6PdadA+Mw+dcOv/AI4n04Leh+Wqp/Fc2WiMjRfMvEEkq48mJLdRjG0Ko/nkGpqqzsddPVXH6uu0WUP8Kq8x+rNtH/oFKIMiUbRkdcVQIaF+YHFMrYlhEbTnzU4boBSfkNWMTWkzqBCf6s4P4AD+tUnZBa8g0tfkm9n/AKCm2N7lcReXqreVgAjJU9D/AIVV7rUztZ6Gjg/3SKk0GsvFMCsV/d/TimTYp3cDNbSNj5RxTJaItI+eJ0J5XkU2ESSaPByRjcOKBmW42Tn0PNV0M3oyGcc00BBMuYs+lMllvw+f9ImX1XP6/wD16J7BR+Jmy1ZHWkezOK5rngELDii47ERXmncoWOKGSZFkkZAcgZwBnHGT6ZwPbNFzSOrsyqlyjNNDgwNHnzUY/NgdTnuPpx+ND01NFBp2ZlKHurGWdvkW9Zmcj/lmhUhMDuMBePemnb5Gqs5WF8MaPLY+H457lcPes1wo/wBgnCn8QM1NWXNPQ6aa5YmfrKgagF/uxqn6lv8A2aqjsBXHMm30FMCQoKVxjCOaYzDEn2m9G77ok2j/AHeQP1qnsNaMsWCeXcXMZ7EH+n9KL6FPcgkGNWHuMfyp9COpr3CgQwEDqpz+dSi3siqVp3FcrtGdr+m7+lUKw3ymWOeKRcbk3D8KLhboYmmNsu5E+v8AOrZES9cktHjPTkUhsyrrl0b1qkQyvOPun2poRD1yD34piJNCO3Udp/iUj+v9KJ7Co/HY6BxWVztse0SLXKfOJldutM1WqInoGivIAQQRkHg5oKMrU0MdpcM7j97tgikc8xs52nn027qdzqpS5lZlTVJLi00+eynKx5TZHc4AVT2Djt6Z6fSrjZu6HKm4u6NnUtceOSK3vdPntIYo0ijfcsiBVUAcj2FZct3e50rY57XCG1ZyDkbkH5RrVrYkgtxmdvpQ9hosMopDKt63lQOR94/Kv1NUikr6GZd2htrSGaNfmU4+ndf1H61Sdy5rsWfl+3JKp+SeLcP0P9TSWwnrqUrwbdShbtux+lUS9zprKKOeOJZFBwp/nWbbRrFXM+7iWOeRVPCngVa2IkkmVSBsl/4Dj9aYuhSeR2clyThSv4VQjDi+XU3PrzV9DNaM0XOQaRRkXPA2/wB1qpEMim+6p7ZxTQiFhgUwC0PlapA46Mw/Xim9YtER0qJnTkCsD0Ge1ypXMfMJlR15pmiIHGKRaIHoLMbxApdLWIAFS7SOD3CgAfzNNHTS0VyRgJilo2GJjUpu53IQBg+pB4+mPekn1R1+pr6mdMutG0240a9hu3Szi+0ojhij7RuBHbBqnpKzCEk9Di9R2m7Z4+EUhuPcIP60wa1sLZcyPSYItP0z6UhmexS6voolZWCfNjPf/wDVn86rZGkF1NS7tFnt3izjcMD2PY1KdimrmDCT9kVZBtmtJcMP9kkg/hyfyrUz2ItTG2WN+wZT+uKEJmmszxBfLbDKeKVrlJshYlmJY5JpiI2HX6UwKLjBemhGD/y/t9K06GfUvFvvVI7mZefeJ9atCZHJzAaOoiLtVAV93yI46o1UZPa/Y608gEdDyK5j01qe6ypXMfJooyx80zWJVkXBoNEVpRQaIzL+ITajJE38Nt5Z+pYj+QFCN9ooxtZvXsrP7T5Amkji3BCxUcgk5I5xyRwQfcVUEnKzOxt8l0Y8ulzeH9L0zU7d5RqN3Et1KGc7JUcBtpH0YfrWs5qTcZbGdGEm/dNGyePUohJbg4uIXZVPVWAPy/gUrKScdGat82omnHLn3GaTGi66qyMHAKkcg9MUhmZpuj2t7CbieHazHKFCVKjtjHtirc2tEXZWLa2N/ZnNpd/aIx/yyueT+Djn8waOaL3QarYztSZUuPOmikgMqmKZGxhsjGVboe3vwOnNXHsJla9Jl03zM5ZeCfcf/qo2ZPQ0TyPakUNIoAgmOJIx65H6UxFeUYVzTEc4pzqD/wC7WvQjqWS3zuPYUgKlyMjNNCIusB+lPqIr5/dE+maYdCsvRh6irMvI6zT383T4H77AD9Rx/Sueasz0aTvBM+hJUrkPlUUpo6LmkWUZkppmiKoT96uRwDk/TvTNEZKDfqN6T22Ln/gIP9aDoeiRS1i3X7EXYArGxDf7pyf/AIqi9mdlBqUbGl4302c+HvDkDRN58WlQqwHzEGKMKfyqK0lz36XCk3BO25kpo6aJdn7MzNFFcTPtJyQobHX/AHRn8TVObmk32HFaMzoo/Ivpoh91CwHuO36Yp3uUgupD9glIPzFCo+p4H86a3Gi9cxvaQwxpwCv61NzVajoIbhoy4fgDJyMAfWi6HYHdWjKzorxng9waa8hWMC6t4oVngg/1UoLJ7HHT8v5Vpe+pna2hYh+aCNvVQf0pdQFIpgUrv5ZYP97+hqkJkV2dsJpoT2OZhOb1m9cj9M1p0M+pYJ/fH/dFAyKflDQhEMfMVPqIq5/dv9asRJZW4liujj5o0DD8/wDAUm7WKhDmua/h9s2LL/ccj+tZ1NzfDv3bH0tItcR8smU5UpmqZSmSg1iysI/nP+63/oJouaowbVMzXjesuPyUVRvLoWf7P/tPGnZ2m+dLUN/dLuqg/gTRFXkjSlPludJrwuU1uG6hhWW3R5Pkzgqj4yBn6Kce3WuKovawcb7neopNNHOWyrNeu6R7I0L/AC4H3nbcRx6DA/GtYLlik+hLVjntVsxZ6qQhJR49wz27Y/DFbJ3RK0MuF0OFlPyllP5HOK0GjqdTs2u4PLWRox32qCT+YIH5VnF2dy3seT3lkqXki2zzgcchzkkjOPwJx+FdTqtBDDxlG7ZdsrPVBYpd2V/LnJVklO5cj/PpTbg3qjHlnHRMhTWLx9QS2u4YIpSwwwyAT2/Pp+NN042vElVZX5ZHS6eS9jCfRcflxWD3OixIw9qYjP1Hia0HrJ/SqQMqavJthIprclnPWZzcKfVjj8citWZliXidT6jFSMbL900xEEX+r/Om9xFRurD3qxGn4eUO92p7qB/OoqdDfD6uQeH2IuriPsVDfl/+uieyFQ0k0fUkida4LnzJWkFFi0VJUoNYlZU/eY7kED6kEUG0WYOnx8XftcOP5UzeT1R0Hg+387xZpEZHHmvJ/wB8RSSD9UFXHqVT1kbGtQIXeJW2FskkHkj69a40j0OZnPfZY7dNsSBQO1WhXuc34rQJ9mmPGA6n8s/0NaRGzk0iSa4to3+5JNGp/FgK1TsSjvNNV9sKXX+sX5HYdCemfx61m/I13RzXg7RZrbXba8uFBS3uwZcZJXHB49s5/Csa1aKm4vsVZyirdDWt7VJL7xAFQCE6lKUGMfKVQjH510RleMWjKzu7nFeONEVbKWZVwYxvVvT1FdFGetjKrC6uVvCE88+l4mVtqsQshI+bvRWST0KoybjqbTjpWZsZOq8TWf8A11FWhMytekwr+1XHczkZNvujMLeXJ8uCfkNW1chdCa4kQlSGwc9+KVmArHP0oArRcbh6GmxFWT75+tUgNDw++2/ZezofzH+TSnsa0HaZPoODe3Te2P1qZ7IdDWcj6ncVwHzBWlXNBcSpInWmaIrOCpDDgg5FBojGsVHnagoGMXJIHsVU/wBaZ0N7HUeA4wfGGmDufPA/8B5auHVeRpS3LfjCzZdl3ESrwt1HdT/kfrXGz0oWvZlCe3ygbHJGaZFtTlPGFuWsIyB92XJ+hVh/UVpApnEXEBsb/wAhutrcp1/uhwwP/fOK2WupK0O6kYA+9Y3OhRuV7O5fT57qXYZopmEmxeGVsYOM8HPXtXNiKHtrNOzLheBLZXlo1o8hYpNNI0jqwwQSeB+AAH4V001yRUexk4ttszL5IdTjmt54G8jIHzcb+c/lWqbjqhcl1ZnPJbJa319BFGkcQkVkVRgAFF/rmrbukxJW0Q7vigZj6822S1PYSg1cSWYmt/vJdndmC/ma0gZy10OysLaFbKPceWGTWTbNwls45AQrKfY0XFcxr3QoCSRGYm9Yzj9OlWpshwizFm0ieKRjGwkU+2D/AIVpzJmbg0ZN1BJFKfMjZRnqRVrUh6MbbzGCdJU6qaN9AT5XzG1okPlK5PUopP1JJ/liom7nRQjY+pHWuA+WK8goLiVpBQaIqyrxTNEZcUfl6ldjs6o/48r/AOyimb7pGppF8ulanaag4JjtZkmfHXYD8wH1XI/GnB2ki4M77XbRRPcQOAyAkDuCOx/KueS5ZOLPQi7q6ObvI8ZXHTipGjn9WsVu4hGxwu9XOO+0g4/SqTsUcf420lnQ38C5ZU8ucDunZvwyfwPtWtOXQJLqXbR/tun29wOsiAkeh7j881ElZ2N4SuiJonVs8s31wBSNLiBGY8x9O5pgx5TFWjM5/Uht1iUAfehRv1Yf0FaLYh6Mq5/eEegpiMLxC3yof7rA/wA60iQzIY+ffx+m8v8Al/kVa0RMVeR0EUFz5Ks9wVTHAJHSoujYURSORslZh/eFAhlxK8fyrKxI65ximMhE03X5W/Aj/GmTa4jTA8SwnB9ORQTylKfTrCfOE8tvVcrVqTRDpJj0gkiLGOZW3Hnevtjt9KTszRScT6accVxnyxA9I0RA60GiRVkHNBrEy7sbNQtn7OHiP1wGH/oJ/Omax2sTA7Tj1oKR13hzVG1HRkt7g5u9PAgY/wB+Hnym+oAKH/cz3pVtbTO2g9LGRHBcWV21vlpbN8lCzEmL/Zyeo9PT6dML3OtqLV0MuU60kQZ0qZyCKopGba2MdnE0MC7YdxZF7Lnkgfjk/jVN33Be6DRD0pl3ImjAoHcgdcVSA5zW026xbt/egcfky/41rH4SJbmahzPN7YFMRg6626E/7wrSO5myhpE9pFfyG8dVAQKAwz15P8hVtNx0CnJJu5vT6xoqn9zbmc+ix/48VKhLqaOoVH8TqP8AV2Py9Pmkxj8hT9n5kc5Vk1lpJCUsoFb/AGmJp8qE5voMkvr6RTtFsn+6h/xp2QuaRRFzeywmQTgAHkBB/hVWiieaXcguZ76JVZpTtbuMU0kDlJEK31wGBMjMAeRnGadkLmlufXTiuA+dRA9ItEDCkaRK8i0GqM7UkJtmZVLPGRIAOpwc4H1xj8aZrHcaSHUOpBUjII7ikVYZbXsml3sd7EjSbAUljXq8ZxuA9xgMPdQO9OykrM2pS5WdlM0c0ayRsHRgGVhyCD0Irmem52mXcDigozZRyapFIrOtMZC4pgQMKCxhhLdqpBsc34nQRajp+epjm/mlaw2JluYUf37hvpVE9Dn9Y5t/q39DVx3M2c1Ln7SxccE5HuOn9K6ehmtzWswqRuAOOKyLRWkTBkX05/X/AOvVIkdH0V/TrQBoxAMoIpFEEcWyaWP+FvmFMm2olzD5loUPUUxvYzLRFYsjrnj8RVMhH144rgPn0QuKk0RA45plogekaIrSCmaIzo08rdDjhTlP93/63T8qGa7kcooBaGn4c1FI1/s+d8Ecwbj1HdB9Ow9OnQ1nUi37yO2lO6szRue9Yo2M+UU0UQMpPQVQyM27n+E0wRHOsVsnmXMsUKf3pGCj9aa12K2Mu716yh+S0DXUp6BBhfz/AMKpRYzkfENxdTaxZvdqiDyJSqL/AAjKVtBe6yJbmRZTeYNQPZXwP++R/XNW1axCd7mVq4Jthgc5qobksm1nw666RFJEhM8QzgdWX0/rVRqajlDTQw7F99u4B5Vc/lVNWZC1H3IAlV/4WHP0NCAbbriRozTYNF60bYdjfhSBdiaaPJVh95f5UJjGsvJHY0w3MaQfZ7wnsardGezPrtxmuE8BFdxikaIrv1oKRC4qWaIgcU0aIqTICc45FBomVJetBRl6q9tFas160aw9D5nQn096qN76GiOJ1HxnqNq6po93dLAGC/6QwlDD2DAsP++vwrX2cH8SNVKS6ir8QtcRcMlhIfV4W/owqPZQLVSQTfEbX2ify10+IqpOVgb+rGqVGFx88tzI0vxjrWs3fk6pqlwiuMxLARCp9QSoB/WtalOMF7iHTcm1z7PY2Ag3bsbnPG48k/j1Nc3M2dSSR0ugaZsHnzD5j0BqJPoBjeMmC65D6Jasfzcf4VrT+AznuYNhGY4Lg4x5iJKP+BLu/rWk+hEdmMW2+1X9jB/z0nXP0HJ/QGiOl2G7PQLm2Vkxj6VimbM4fXvDrQzveWMeSQfMiH8Q9R7/AM63jUvozKUOqOWH721K94zjpzitHozMbG3CSd0+V/p2NMaNGWH5BInIIqRNdR1tMHG1uvY0DTJnjIGejDmmmNmZqluHj8xapMiS6n1e1cTPn0Qyc0i4laQUGhA4pMtFeQdaSNEV5BTLRm6jMtrayzsN2wcKOrHsB7k4H41UVdlo8l1G4mvrp5riQyMTxk8AegHYVpfsdCSRQnh3bAeMuv8AOhMdhs8WxsUIZCseWdR3Qj9KpOzCxjaZBLc3dhBA+yV5FCt/dOev4V0NpXbNpK9OCR6p4eIa4aK7RRcxnDAdM+o9j1rhlpsapux2aH5OKxsM8+8csW1W4C8sLRIwP9os/wDiK6qeyM57i6lCtveXkC9IlijH0Ea0r3swehD4ejEniWzH/POOST9Av/s1U/hYLdHcSdKxRqxgjEg2kdaYWOW8S+GBOz3VgFjusfMp4WUeh9D71pCp0ZnKNzgOba7ZZo2RlO2WNhyBW62MtmbFqBGvl53RkZU+opMZWvLYxMZI/uHk+1NEtE1pdt5exuvbNFik7kUrAhlI+U9vSmJn1M/SuQ+dRA9JmiK8gpGiK70i0QvzSLRVfrTLRzfikmSWwtRwjy+bJ/uqyr/6FIh/Cne0WzWnqzzxDbwXkds2WmcZBxwKuzaub36C38IJR+6uv480Ior3MIf60wsVY4dk6nOcg07gQ+ArMSeJAGH/AB6o7D652/1Na13aHqbU9VHyPRbyya3ube5QfvG4Pv3rkT0sbdTft33xqw7ioA4eRP7S+IU0QGYYHR3/AOAKCP8Ax410r3aaZl8UrCa3/wAhvUx/00j/APRS1K2QPdjvC0X/ABUTuf4bZl/Nl/wpyfuhHc7BlyDWRqNT7woGkTSR7loCxy/iXQINTj3EeXcoPklA5HsfUVtCbRnOKZxMEctpIbG8XZKnKnsR6g+lbb6oytYtKTkIaQEMtrG+d64HrtppgilNaKMiOYqfY5FVcTR//9k" />
  </div>
  <div id="bar" class="bar" data-tauri-drag-region>
    <div class="gradient" />
    {#each $figure as pi, i}
      <div class={`box ${a_count === i ? 'ac' : ''}`}>
        <Avatar
          onclick={() => {
            change(i)
          }}
          iName="{pi.name},"
          base64={`${pi.avatar}`} />
      </div>
    {/each}
    <div class={`box ${a_count === -2 ? 'ac' : ''}`}>
      <PlusButton
        onclick={() => {
          change(-2)
        }} />
    </div>
  </div>
</div>

<style lang="postcss">
  .main {
    display: flex;
    flex-direction: column;
    height: 100%;
    /* background-color: var(--bg-color0); */
    width: var(--side-width);
    position: relative;
    --transition: 200ms;
    --shadow-size: var(--icon-padding);
    --shadow-color: color-mix(in srgb, var(--bg-ac-color0) 60%, var(--shadow));
    --hover--inner-color: color-mix(in srgb, var(--bg-ac-color0) 95%, #ffff);

    

    &.ov:not(.macos)::before {
      box-shadow: calc(0px - var(--sh-size)) calc(0px - var(--sh-size)) 0
        var(--sh-size) var(--bg-color0);
    }

    --icon-padding: calc(var(--side-margin) * 0.7);
  }

  .bar {
    box-sizing: border-box;
    background-color: transparent;
    flex-grow: 1;
    top: 0;
    display: flex;
    flex-flow: column;
    padding: var(--icon-padding);
    padding-right: 0;
    padding-bottom: var(--radius-size);
    /*padding-top: 0;*/

    overflow: scroll;

    &::-webkit-scrollbar {
      display: none;
    }

    & > .gradient {
      position: absolute;
      height: var(--icon-padding);
      /*background-color: #f00;*/
      background: linear-gradient(var(--bg-color0) 40%, transparent);
      width: calc(var(--side-width) - var(--icon-padding) * 3);
      margin-top: calc(0px - var(--icon-padding));
      margin-left: var(--icon-padding);
      z-index: 2;
    }
  }

  .box {
    padding: var(--icon-padding);
    padding-top: 0;
    border-radius: var(--radius-size) 0 0 var(--radius-size);
    position: relative;
    /*margin-bottom: calc(var(--side-margin) * 0.5);*/
    background-color: transparent;
    transition: all var(--transition);

    &::before {
      content: '';
      position: absolute;
      width: var(--radius-size);
      height: var(--radius-size);
      background-color: transparent;
      top: calc(0px - var(--radius-size));
      right: 0;
      border-bottom-right-radius: var(--radius-size);
      --sh-size: calc(var(--radius-size) * 0.25);
      box-shadow: var(--sh-size) var(--sh-size) 0 var(--sh-size) transparent;
      transition: all var(--transition);
    }

    &::after {
      content: '';
      position: absolute;
      width: var(--radius-size);
      height: var(--radius-size);
      background-color: transparent;
      bottom: calc(0px - var(--radius-size));
      right: 0;
      border-top-right-radius: var(--radius-size);
      --sh-size: calc(var(--radius-size) * 0.25);
      box-shadow: var(--sh-size) calc(0px - var(--sh-size)) 0 var(--sh-size)
        transparent;
      transition: all 220ms;
    }

    &.main-box {
      margin-left: var(--icon-padding);
      padding-top: var(--icon-padding);
      padding-bottom: 0;
      margin-bottom: 0;
      z-index: 3;

      &::before {
        display: none;
        box-shadow: none;
      }

      &::after {
        z-index: -1;
      }
    }

    &.ac {
      background-color: var(--bg-color1);
      padding-top: var(--icon-padding);
      position: sticky;
      top: 0;
      bottom: 0;
      z-index: 1;

      & + .box {
        padding-top: var(--icon-padding);
      }

      &::before {
        box-shadow: var(--sh-size) var(--sh-size) 0 var(--sh-size)
          var(--bg-color1);
      }

      &::after {
        box-shadow: var(--sh-size) calc(0px - var(--sh-size)) 0 var(--sh-size)
          var(--bg-color1);
      }

      &.main-box {
        padding-bottom: var(--icon-padding);
      }
    }
  }
</style>
